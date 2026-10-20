use crate::assembler::token::TokenType::MemPtr;

use super::token::*;
use super::utils::*;

// 0x00fa
fn read_hex_literal(word: &str) -> Token {
    Token::new(TokenType::Hex, word.to_string())
}

// 0 - 31
fn read_literal(word: &str) -> Token {
    Token::new(TokenType::Literal, word.to_string())
}

// :end
fn read_label(word: &str) -> Token {
    Token::new(TokenType::Label, word.to_string())
}

// [0xfa01]
fn read_memory_pointer(word: &str) -> Token {
    let word = word
        .strip_prefix("[")
        .unwrap()
        .strip_suffix("]")
        .unwrap()
        .to_string();
    Token::new(MemPtr, word)
}

pub fn tokenize(path: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let asm = read_asm(path);
    for line in asm {
        for word in line.split_whitespace() {
            let c = word.chars().nth(0).unwrap();
            let cc = if word.len() > 1 {
                word.chars().nth(1).unwrap()
            } else {
                '0'
            };
            if c == ':' {
                read_label(word);
            } else if c.is_alphabetic() {
                println!("found identifier: {word}");
            } else if c == '[' {
                tokens.push(read_memory_pointer(word));
            } else if c == '0' && cc == 'x' {
                if let Some(cc) = word.chars().nth(1) {
                    if cc == 'x' {
                        tokens.push(read_hex_literal(word));
                    }
                }
            } else if c.is_numeric() {
                tokens.push(read_literal(word));
            }
        }
    }
    tokens
}
