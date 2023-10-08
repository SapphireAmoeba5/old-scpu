use anyhow::{Result, bail};
use crate::instructions::Instruction;

pub enum Token {
    Instruction(Instruction),
    
}
pub fn is_special_token(ch: char) -> bool {
    ch == '('
        || ch == ')'
        || ch == '['
        || ch == ']'
        || ch == '{'
        || ch == '}'
        || ch == '+'
        || ch == '-'
        || ch == '*'
        || ch == '/'
        || ch == '>'
        || ch == '<'
        || ch == ';'
        || ch == ','
        || ch == ':'
        || ch == '='
    // || ch == '.'
}

pub fn lex_string<'a>(string: &'a str) -> Vec<&'a str> {
    let mut lexed: Vec<&'a str> = Vec::new();

    let mut start: usize = 0;
    let mut first_character_of_token: Option<char> = None;

    for (i, ch) in string.chars().enumerate() {
        if ch == '\n' {
            let token = string[start..i].trim();
            if !token.is_empty() {
                lexed.push(token);
            }
            let special_token = &string[i..i + 1];
            lexed.push(special_token);
            start = i + 1;
            first_character_of_token = None;
        }
        else if ch.is_whitespace() {
            let token = &string[start..i].trim();
            if !token.is_empty() {
                lexed.push(token);
            }
            start = i + 1;
            first_character_of_token = None;
        } else if is_special_token(ch) {
            let token = string[start..i].trim();
            if !token.is_empty() {
                lexed.push(token);
            }
            let special_token = &string[i..i + 1];
            lexed.push(special_token);
            start = i + 1;
            first_character_of_token = None;
        } else if ch == '.' && first_character_of_token.is_some_and(|ch| !ch.is_numeric()) {
            let token = string[start..i].trim();
            if !token.is_empty() {
                lexed.push(token);
            }
            let special_token = &string[i..i + 1];
            lexed.push(special_token);
            start = i + 1;
            first_character_of_token = None;
        }

        if first_character_of_token.is_none() && !ch.is_whitespace() {
            first_character_of_token = Some(ch);
        }
    }

    lexed
}

pub fn tokenize(source: String) -> Result<Vec<Token>> {
    let lexed = lex_string(&source);
    bail!("Not implemented\n'{:?}'", lexed);
}
