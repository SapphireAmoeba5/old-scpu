use anyhow::{Result, bail};

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

pub struct TokenIterator<'a> {
    tokens: &'a[&'a str],
    current_token: usize,
}

impl<'a> TokenIterator<'a> {
    pub fn new(tokens: &'a[&'a str]) -> Self {
        Self {
            tokens,
            current_token: 0,
        }
    }

    pub fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.current_token).and_then(|x| Some(*x))
    }

    pub fn rewind(&mut self, n: usize) {
        match self.current_token.checked_sub(n) {
            Some(n) => self.current_token = n,
            None => self.current_token = 0,
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.current_token);
        self.current_token += 1;
        token.and_then(|x| Some(*x))
    }
}
