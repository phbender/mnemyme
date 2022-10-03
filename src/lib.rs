//! This crate can be used to transform `u32` values (IDs) to user-friendly word triples.
//! 
//! The word list is based on [the EFF large word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt)
//! which can encode nearly 13 bits. Therefore, the base for this crate is 12 bits, which effectively uses only `4096`
//! items from the list. To encode a `u32` value, three words are required.
//! 
//! The wordlist is statically compiled into the library and cannot be changed.

use lazy_static::lazy_static;
use std::collections::HashMap;
use thiserror::Error;



#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("")]
    DecodeError,
}
struct Words {
    forward: Vec<String>,
    reverse: HashMap<String, u32>,
}

lazy_static! {
    static ref WORDS: Words = {
        let forward: Vec<String> = include_str!("./wordlist.txt")
            .split('\n')
            .map(|v| v.into())
            .take(4096)
            .collect();
        let mut reverse = HashMap::new();
        forward.iter().enumerate().for_each(|(i, v)| {
            reverse.insert(v.into(), i as u32);
        });
        Words { forward, reverse }
    };
}

pub fn encode(n: &u32) -> String {
    let words: Vec<String> = [(n >> 20) & 4095, (n >> 8) & 4095, (n) & 255]
        .iter()
        .map(|idx| WORDS.forward.get(*idx as usize).expect("").clone())
        .collect();

    words.join("-")
}

fn find_index(w: &&str) -> Result<u32, Error> {
    let result = WORDS.reverse.get(*w);
    match result {
        Some(idx) => Ok(*idx),
        None => Err(Error::DecodeError),
    }
}

/// Decodes a words triple.
/// 
/// This method will split the string at `-` and decode a `u32` value.
/// If not all words can be found in the word list, it will return a `Error::DecodeError`.
pub fn decode(w: &str) -> Result<u32, Error> {
    let words: Vec<&str> = w.split("-").collect();

    let maybe_indices: Result<Vec<u32>, _> = words.iter().map(find_index).collect();

    match maybe_indices {
        Ok(indices) => {
            let a = indices[0];
            let b = indices[1];
            let c = indices[2];

            Ok((a << 20) | (b << 8) | c)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_correct_len() {
        assert_eq!(WORDS.forward.len(), 4096);
        assert_eq!(WORDS.reverse.len(), 4096);
    }

    #[test]
    fn it_works() {
        assert_eq!(encode(&12), "abacus-abacus-abridge");
        assert_eq!(decode("abacus-abacus-abridge"), Ok(12));

        assert_eq!(encode(&17453), "abacus-affluent-activator");
        assert_eq!(decode("abacus-affluent-activator"), Ok(17453));
    }
}
