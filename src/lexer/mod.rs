mod token;
use crate::error::err_trait::CompilerError;
use token::{Ints, TokenKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Normal,
    String,
    IdentifierDetect,
    Comment,
    DocComment,
}

// TODO: Fix this
pub fn lexer(input: &str) -> Result<Vec<token::TokenKind>, ()> {
    use token::TokenKind;

    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;
    let mut state = State::Normal;

    while index < chars.len() {
        match state {
            State::Normal => {
                let c = chars[index];

                if c.is_whitespace() {
                    index += 1;
                    continue;
                }

                if is_identifier_start(c) {
                    state = State::IdentifierDetect;
                    continue;
                }

                if c.is_ascii_digit() {
                    let (token, next_index) = lex_number(&chars, index);
                    tokens.push(token);
                    index = next_index;
                    continue;
                }

                match c {
                    '"' => {
                        state = State::String;
                        index += 1;
                    }
                    '\'' => {
                        index = lex_char(&chars, index)?;
                        tokens.push(TokenKind::Char('\''));
                    }
                    '/' => {
                        if matches_char(&chars, index + 1, '/') {
                            if matches_char(&chars, index + 2, '/') {
                                state = State::DocComment;
                                index += 3;
                            } else {
                                state = State::Comment;
                                index += 2;
                            }
                        } else {
                            tokens.push(TokenKind::Div);
                            index += 1;
                        }
                    }
                    '(' => {
                        tokens.push(TokenKind::LParen);
                        index += 1;
                    }
                    ')' => {
                        tokens.push(TokenKind::RParen);
                        index += 1;
                    }
                    '{' => {
                        tokens.push(TokenKind::LBrace);
                        index += 1;
                    }
                    '}' => {
                        tokens.push(TokenKind::RBrace);
                        index += 1;
                    }
                    '[' => {
                        tokens.push(TokenKind::LBracket);
                        index += 1;
                    }
                    ']' => {
                        tokens.push(TokenKind::RBracket);
                        index += 1;
                    }
                    ',' => {
                        tokens.push(TokenKind::Comma);
                        index += 1;
                    }
                    ':' => {
                        tokens.push(TokenKind::Colon);
                        index += 1;
                    }
                    ';' => {
                        tokens.push(TokenKind::Semicolon);
                        index += 1;
                    }
                    '.' => {
                        if matches_char(&chars, index + 1, '.') {
                            tokens.push(TokenKind::Range);
                            index += 2;
                        } else {
                            tokens.push(TokenKind::Dot);
                            index += 1;
                        }
                    }
                    '@' => {
                        tokens.push(TokenKind::At);
                        index += 1;
                    }
                    '+' => {
                        tokens.push(TokenKind::Add);
                        index += 1;
                    }
                    '-' => {
                        if matches_char(&chars, index + 1, '>') {
                            tokens.push(TokenKind::Arrow);
                            index += 2;
                        } else {
                            tokens.push(TokenKind::Sub);
                            index += 1;
                        }
                    }
                    '*' => {
                        tokens.push(TokenKind::Mul);
                        index += 1;
                    }
                    '%' => {
                        tokens.push(TokenKind::Mod);
                        index += 1;
                    }
                    '=' => {
                        if matches_char(&chars, index + 1, '=')
                            && matches_char(&chars, index + 2, '>')
                        {
                            tokens.push(TokenKind::MatchArrow);
                            index += 3;
                        } else if matches_char(&chars, index + 1, '=') {
                            tokens.push(TokenKind::EqEq);
                            index += 2;
                        } else if matches_char(&chars, index + 1, '>') {
                            tokens.push(TokenKind::FatArrow);
                            index += 2;
                        } else {
                            tokens.push(TokenKind::Eq);
                            index += 1;
                        }
                    }
                    '!' => {
                        if matches_char(&chars, index + 1, '=') {
                            tokens.push(TokenKind::Neq);
                            index += 2;
                        } else {
                            tokens.push(TokenKind::Not);
                            index += 1;
                        }
                    }
                    '&' => {
                        if matches_char(&chars, index + 1, '&') {
                            tokens.push(TokenKind::And);
                            index += 2;
                        } else {
                            tokens.push(TokenKind::Amp);
                            index += 1;
                        }
                    }
                    '|' => {
                        if matches_char(&chars, index + 1, '|') {
                            tokens.push(TokenKind::Or);
                            index += 2;
                        } else if matches_char(&chars, index + 1, '>') {
                            tokens.push(TokenKind::Pipe);
                            index += 2;
                        } else {
                            return Err(());
                        }
                    }
                    '<' => {
                        if matches_char(&chars, index + 1, '=') {
                            tokens.push(TokenKind::Le);
                            index += 2;
                        } else {
                            tokens.push(TokenKind::Lt);
                            index += 1;
                        }
                    }
                    '>' => {
                        if matches_char(&chars, index + 1, '=') {
                            tokens.push(TokenKind::Ge);
                            index += 2;
                        } else {
                            tokens.push(TokenKind::Gt);
                            index += 1;
                        }
                    }
                    _ => return Err(()),
                }
            }
            State::String => {
                let (token, next_index) = lex_string(&chars, index).unwrap_or_else(|()| {
                    let err = crate::error::syntax_error::SyntaxError::new(
                        "Expected closing quote for string".to_string(),
                        0,
                        0,
                    );
                    err.msg();
                    std::process::exit(1)
                });
                tokens.push(token);
                index = next_index;
                state = State::Normal;
            }
            State::IdentifierDetect => {
                let (token, next_index) = lex_identifier(&chars, index);
                tokens.push(token);
                index = next_index;
                state = State::Normal;
            }
            State::Comment | State::DocComment => {
                while index < chars.len() && chars[index] != '\n' {
                    index += 1;
                }
                state = State::Normal;
            }
        }
    }

    if state == State::String {
        return Err(());
    }

    Ok(tokens)
}

fn lex_identifier(chars: &[char], start: usize) -> (token::TokenKind, usize) {
    use token::TokenKind;

    let mut end = start + 1;
    while end < chars.len() && is_identifier_continue(chars[end]) {
        end += 1;
    }

    let ident: String = chars[start..end].iter().collect();
    let token = match ident.as_str() {
        "use" => TokenKind::Use,
        "var" => TokenKind::Var,
        "val" => TokenKind::Val,
        "fun" => TokenKind::Fun,
        "for" => TokenKind::For,
        "in" => TokenKind::In,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        "ext" => TokenKind::Ext,
        "typ" => TokenKind::Typ,
        "imp" => TokenKind::Imp,
        "enm" => TokenKind::Enm,
        "self" => TokenKind::SelfKw,
        "goto" => TokenKind::Goto,
        "lbl" => TokenKind::Lbl,
        "true" => TokenKind::Bool(true),
        "false" => TokenKind::Bool(false),
        _ => TokenKind::Identifier(ident.to_string()),
    };

    (token, end)
}

fn lex_number(chars: &[char], start: usize) -> (token::TokenKind, usize) {
    // returns end, which is the index
    use token::TokenKind;

    let mut num: String = String::new();

    let mut end = start;
    while end < chars.len() && is_number(chars[end]) {
        num.push(chars[end]);
        end += 1;
    }

    (
        TokenKind::Int(token::Ints::I32(num.parse().unwrap_or(0))),
        end,
    )
}

fn lex_string(chars: &[char], start: usize) -> Result<(token::TokenKind, usize), ()> {
    let mut index = start;
    let mut escaped = false;

    while index < chars.len() {
        let c = chars[index];

        if escaped {
            escaped = false;
            index += 1;
            continue;
        }

        match c {
            '\\' => {
                escaped = true;
                index += 1;
            }
            '"' => {
                return Ok((
                    TokenKind::String(chars[start..index].iter().collect()),
                    index + 1,
                ));
            }
            _ => index += 1,
        }
    }

    Err(())
}

fn lex_char(chars: &[char], start: usize) -> Result<usize, ()> {
    let mut index = start + 1;

    if index >= chars.len() {
        return Err(());
    }

    if chars[index] == '\\' {
        index += 2;
    } else {
        index += 1;
    }

    if matches_char(chars, index, '\'') {
        Ok(index + 1)
    } else {
        Err(())
    }
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

fn is_identifier_continue(c: char) -> bool {
    c == '_' || c.is_alphanumeric()
}

fn matches_char(chars: &[char], index: usize, expected: char) -> bool {
    chars.get(index).copied() == Some(expected)
}

fn matches_char_by(chars: &[char], index: usize, predicate: impl FnOnce(char) -> bool) -> bool {
    chars.get(index).copied().is_some_and(predicate)
}

fn is_number(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::lexer;
    use super::token::{self, TokenKind};

    #[test]
    fn lexes_variable_declaration() {
        let tokens = lexer("val a = 5").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Val,
                TokenKind::Identifier("a".to_string()),
                TokenKind::Eq,
                TokenKind::Int(token::Ints::I32(5)),
            ]
        );
    }

    #[test]
    fn lexes_range_and_block() {
        let tokens = lexer("for i in 0..10 { } ").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::For,
                TokenKind::Identifier("i".to_string()),
                TokenKind::In,
                TokenKind::Int(token::Ints::I32(0)),
                TokenKind::Range,
                TokenKind::Int(token::Ints::I32(10)),
                TokenKind::LBrace,
                TokenKind::RBrace,
            ]
        );
    }

    #[test]
    fn skips_comments_and_handles_strings() {
        let tokens = lexer("println(\"hello ${name}\") // comment").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Identifier("println".to_string()),
                TokenKind::LParen,
                TokenKind::String("hello ${name}".to_string()),
                TokenKind::RParen,
            ]
        );
    }

    #[test]
    fn lexes_member_access_and_self_parameter() {
        let tokens = lexer("fun eat(&self) { self.age }").unwrap();
        assert_eq!(
            tokens,
            vec![
                TokenKind::Fun,
                TokenKind::Identifier("eat".to_string()),
                TokenKind::LParen,
                TokenKind::Amp,
                TokenKind::SelfKw,
                TokenKind::RParen,
                TokenKind::LBrace,
                TokenKind::SelfKw,
                TokenKind::Dot,
                TokenKind::Identifier("age".to_string()),
                TokenKind::RBrace,
            ]
        );
    }
}
