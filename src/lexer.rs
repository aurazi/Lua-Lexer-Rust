use crate::token::{Token,TokenData};
use std::iter::{Peekable, Iterator};

pub struct Lexer<T> 
where
    T: Iterator<Item=char>
{
    source: Peekable<T>,
}

impl <T> Lexer<T> 
where
    T: Iterator<Item=char>
{
    pub fn new(source_iterator: T) -> Self {
        Self {
            source: source_iterator.peekable()
        }
    }
    fn parse_identifier(&mut self) -> Token {
        let mut parsed_string = String::new();
        while let Some(char) = self.source.peek() {
            match char {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let consumed_char = self.source.next().unwrap();
                    parsed_string.push(consumed_char);
                },
                _ if char.is_ascii_digit() => {
                    let consumed_char = self.source.next().unwrap();
                    parsed_string.push(consumed_char);
                },
                _ => break,
            }
        }
        let token = match parsed_string.as_str() {
            "local" => Token::KeywordLocal,
            "if" => Token::KeywordIf,
            "do" => Token::KeywordDo,
            "then" => Token::KeywordThen,
            "while" => Token::KeywordWhile,
            "repeat" => Token::KeywordRepeat,
            "until" => Token::KeywordUntil,
            "end" => Token::KeywordEnd,
            "else" => Token::KeywordElse,
            "elseif" => Token::KeywordElseif,
            "true" => Token::LiteralTrue,
            "false" => Token::LiteralFalse,
            "nil" => Token::LiteralNil,
            "in" => Token::KeywordIn,
            "break" => Token::KeywordBreak,
            "continue" => Token::KeywordContinue,
            "return" => Token::KeywordReturn,
            "and" => Token::KeywordAnd,
            "or" => Token::KeywordOr,
            "not" => Token::KeywordNot,
            "for" => Token::KeywordFor,
            "function" => Token::KeywordFunction,
            _ => Token::Identifier(TokenData{source: parsed_string})
        };
        token
    }
    fn parse_till(&mut self, look_out: char) -> String {
        let mut parsed_string = String::new();
        while let Some(char) = self.source.peek() {
            if char == &look_out {
                let _ = self.source.next().unwrap();
                break
            }
            parsed_string.push(self.source.next().unwrap());
        }
        parsed_string
    }
    fn parse_number(&mut self) -> String {
        let mut parsed_string = String::new();
        while let Some(char) = self.source.peek() {
            if !char.is_ascii_digit() {
                break
            }
            parsed_string.push(self.source.next().unwrap());
        }
        parsed_string
    }
    pub fn to_tokens(mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(char) = self.source.peek() {
            match char {
                '0'..='9' => {
                    let parsed_number = self.parse_number();
                    tokens.push(Token::LiteralNumber(TokenData{source: parsed_number}));
                },
                '"' | '\'' => {
                    let temp = self.source.next().unwrap();
                    let parsed_string = self.parse_till(temp);
                    tokens.push(Token::String(TokenData{source: parsed_string}));
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let parsed_identifier = self.parse_identifier();
                    tokens.push(parsed_identifier);
                },
                ' ' | '\n' | '\r' => {
                    let _ = self.source.next().unwrap();
                },
                '#' => {
                    let _ = self.source.next().unwrap();
                    tokens.push(Token::OperatorLen);
                },
                '(' => {
                    let _ = self.source.next().unwrap();
                    tokens.push(Token::OpenedBracket);
                },
                ')' => {
                    let _ = self.source.next().unwrap();
                    tokens.push(Token::ClosedBracket);
                },
                '[' => {
                    let _ = self.source.next().unwrap();
                    tokens.push(Token::OpenedSquareBracket);
                },
                ']' => {
                    let _ = self.source.next().unwrap();
                    tokens.push(Token::ClosedSquareBracket);
                },
                '{' => {
                    let _ = self.source.next().unwrap();
                    tokens.push(Token::OpenedCurlyBracket);
                },
                '}' => {
                    let _ = self.source.next().unwrap();
                    tokens.push(Token::ClosedCurlyBracket);
                },
                '-' => {
                    let temp = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        match next_char {
                            &next if next == temp => {
                                let _ = self.source.next().unwrap();
                                let parsed_string = self.parse_till('\n');
                                tokens.push(Token::Comment(TokenData{source: parsed_string}));
                                continue
                            },
                            &next if next == '=' => {
                                let _ = self.source.next().unwrap();
                                tokens.push(Token::OperatorSubtractionAssignment);
                                continue
                            },
                            &next if next != ' ' => {
                                tokens.push(Token::OperatorUnary);
                                continue
                            },
                            _ => (),
                        }
                    }
                    tokens.push(Token::OperatorSubtraction);
                },
                '=' => {
                    let temp = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &temp {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorEquality);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorAssignment);
                },
                '>' => {
                    let _ = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &'=' {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorGreaterOrEqual);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorGreater);
                },
                '<' => {
                    let _ = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &'=' {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorLowerOrEqual);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorLower);
                },
                '+' => {
                    let _ = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &'=' {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorAdditionAssignment);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorAddition);
                },
                '/' => {
                    let _ = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &'=' {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorDivisionAssignment);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorDivision);
                },
                '*' => {
                    let _ = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &'=' {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorMultiplicationAssignment);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorMultiplication);
                },
                '%' => {
                    let _ = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &'=' {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorModulusAssignment);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorModulus);
                },
                '^' => {
                    let _ = self.source.next().unwrap();
                    let next_char = self.source.peek();
                    if let Some(next_char) = next_char {
                        if next_char == &'=' {
                            let _ = self.source.next().unwrap();
                            tokens.push(Token::OperatorPowerAssignment);
                            continue;
                        }
                    }
                    tokens.push(Token::OperatorPower);
                },
                _ => {
                    let c = self.source.next().unwrap();
                    tokens.push(Token::UnknownCharacter(c));
                }
            }
        }
        println!();
        tokens
    }
}