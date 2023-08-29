use std::{fmt::Debug, marker::PhantomData};
use strum::IntoEnumIterator;
use anyhow::{ anyhow, Error };
use itertools::{ multipeek, structs::MultiPeek };

pub trait Tokenizable {
    fn claim(&self, buffer: &mut MultiPeek<impl Iterator<Item = char>>) -> Option<String>;
}

pub struct Tokenizer<'a, Kind> {
    _marker: PhantomData<Kind>,

    faulted: bool,
    start: usize,
    line: usize,
    column: usize,
    it: MultiPeek<std::str::Chars<'a>>,
}

impl<'a, Kind> Tokenizer<'a, Kind> {
    pub fn new(input: &'a str) -> Self {
        Self { _marker: Default::default(), faulted: false, start: 0, line: 0, column: 0, it: multipeek(input.chars()) }
    }
}

impl<'a, Kind: Tokenizable + IntoEnumIterator> Iterator for Tokenizer<'a, Kind> {
    type Item = Result<Token<Kind>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.faulted {
            return None;
        }

        match next(&mut self.it) {
            // Probably failed to tokanize a character
            Err(err) => {
                self.faulted = true;

                Some(Err(err))
            },

            // End reached
            Ok(None) => {
                None
            },
            
            Ok(Some((kind, value))) => {
                let length = value.len();
                let start = self.start;

                self.start += length;

                Some(Ok(Token { kind: kind, start: start, length: length, line: 0, column: 0, value: value }))
            },
        }
    }
}

pub fn tokenize<'a, Kind: Tokenizable + Default + IntoEnumIterator + 'a>(input: &'a str) -> impl Iterator<Item = Result<Token<Kind>, Error>> + 'a {
    Tokenizer::new(input)
}

fn next<Kind: Tokenizable + IntoEnumIterator>(it: &mut MultiPeek<impl Iterator<Item = char>>) -> Result<Option<(Kind, String)>, Error> {
    if let None = it.peek() {
        return Ok(None);
    }

    for candidate in Kind::iter() {
        it.reset_peek();

        if let Some(value) = candidate.claim(it) {
            return Ok(Some((candidate, value)))
        }
    }
    
    Err(anyhow!("Some kind of syntax error, dunno what yet"))
}

#[derive(Debug)]
pub struct Token<Kind> {
    pub kind: Kind,
    pub start: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
    pub value: String,
}