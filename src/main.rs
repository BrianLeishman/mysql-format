extern crate htmlescape;
extern crate time;

use htmlescape::encode_minimal;
use std::fs;
use time::PreciseTime;

enum Token {
    Unset = 0,
    Word = 1,
    Name = 2,
    Symbol = 3,
    Number = 4,
    String = 5,
    Hex = 6,
    HexString = 7,
}

const MYSQL: &str = "select`users`.`UserID`
from`users`
join`companies`using(`CompanyID`)
where`users`.`Email`='<phil@redshift.com>'
and`companies`.`NetworkID`=x'1541A488C87419F2'
and`users`.`__Active`<>0 
order by`users`.`__Added`desc
limit 1;";

fn main() {
    let bytes = MYSQL.as_bytes();
    let mut s = String::from("<pre><code>");
    let len = MYSQL.len();
    let mut cur_token = Token::Unset;
    let mut prev_token = Token::Unset;
    let mut token_start = 0usize;
    let mut token_end = 0usize;
    let mut escaped = false;
    let mut quote = '\'';
    let mut p = 0;
    let mut prev_str = String::new();
    let mut skip = 0u8;
    // let mut prev_char = '+';

    let start = PreciseTime::now();
    for (i, b) in bytes.iter().enumerate() {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        macro_rules! push_newline {
            () => {
                s.push_str(&format!("\n{}", " ".repeat(4 * p)));
            };
        }

        // push a token to the formatted output
        macro_rules! push_token {
            () => {
                match cur_token {
                    Token::Unset => {
                        break;
                    }
                    Token::Word => {
                        match prev_token {
                            Token::Word => s.push(' '),
                            _ => {}
                        }
                        let word = &MYSQL[token_start..=token_end].to_ascii_lowercase();
                        match word.as_str() {
                            "from" | "where" | "and" | "or" | "order" | "group" | "having"
                            | "limit" | "straight_join" | "cross" | "natural" | "union" => {
                                push_newline!()
                            }
                            // "select" => match prev_token {
                            //     Token::Symbol => {
                            //         if prev_char == '(' {
                            //             push_newline!();
                            //         }
                            //     }
                            //     _ => {}
                            // },
                            "left" | "right" => match prev_token {
                                Token::Word => match prev_str.as_str() {
                                    "natural" => {}
                                    _ => push_newline!(),
                                },
                                _ => push_newline!(),
                            },
                            "join" => match prev_token {
                                Token::Word => match prev_str.as_str() {
                                    "inner" | "cross" | "outer" | "left" | "right"
                                    | "natural" => {}
                                    _ => push_newline!(),
                                },
                                _ => push_newline!(),
                            },
                            _ => {}
                        }
                        s.push_str("<b style=\"color:#1976d2\">");
                        s.push_str(word);
                        s.push_str("</b>");
                        prev_str = word.clone();
                    }
                    Token::String => {
                        s.push_str("<span style=\"color:#4caf50\">'");
                        if quote == '\'' {
                            s.push_str(&encode_minimal(&MYSQL[token_start..=token_end]));
                        } else {
                            s.push_str(&encode_minimal(
                                &MYSQL[token_start..=token_end]
                                    .replace(
                                        &format!("{}{}", "\\", &quote.to_string()).to_string(),
                                        &quote.to_string(),
                                    ).replace("'", "\\'"),
                            ));
                        }
                        s.push_str("'</span>");
                    }
                    Token::Name => {
                        match prev_token {
                            Token::Name => s.push(' '),
                            _ => {}
                        }
                        s.push_str("<span style=\"color:#795548\">`");
                        s.push_str(&encode_minimal(&MYSQL[token_start..=token_end]));
                        s.push_str("`</span>");
                    }
                    Token::Number => {
                        match prev_token {
                            Token::Word => s.push(' '),
                            _ => {}
                        }
                        s.push_str("<span style=\"color:#ff9800\">");
                        s.push_str(&MYSQL[token_start..=token_end]);
                        s.push_str("</span>");
                    }
                    Token::Hex | Token::HexString => {
                        match prev_token {
                            Token::Word => s.push(' '),
                            _ => {}
                        }
                        s.push_str("<span style=\"color:#673ab7;background-color:#f0f0f0\">");
                        s.push_str(&MYSQL[token_start..=token_end]);
                        s.push_str("</span>");
                    }
                    Token::Symbol => {
                        s.push_str("<b>");
                        s.push_str(&encode_minimal(&format!("{}", bytes[i] as char)));
                        s.push_str("</b>");

                        // prev_char = bytes[i] as char;
                    }
                }
                prev_token = cur_token;
                cur_token = Token::Unset;
            };
        }

        // make sure current byte is valid for cur_token
        match cur_token {
            Token::Unset => {}
            Token::Word => match *b as char {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {}
                _ => {
                    token_end = i - 1;
                    push_token!();
                }
            },
            Token::Number => match *b as char {
                '0'...'9' | '.' => {}
                _ => {
                    token_end = i - 1;
                    push_token!();
                }
            },
            Token::Name | Token::String => {
                if !escaped {
                    match *b as char {
                        b if b == quote => {
                            token_end = i - 1;
                            push_token!();
                            continue;
                        }
                        '\\' => escaped = true,
                        _ => {}
                    }
                } else {
                    escaped = false;
                }
            }
            Token::HexString => match *b as char {
                '\'' => {
                    token_end = i - 1;
                    push_token!();
                    continue;
                }
                _ => {}
            },
            Token::Symbol => {}
            Token::Hex => match *b as char {
                'a'...'f' | 'A'...'F' | '0'...'9' => {}
                _ => {
                    token_end = i - 1;
                    push_token!();
                }
            },
        }

        // Set token if we're currently unset
        match cur_token {
            Token::Unset => match *b as char {
                'a'...'z' | 'A'...'Z' => {
                    match *b as char {
                        'x' | 'X' => {
                            if i + 1 < len {
                                let next_b = bytes[i + 1] as char;
                                if next_b == '\'' {
                                    cur_token = Token::HexString;
                                    token_start = i;
                                    skip = 1;
                                }
                            }

                            continue;
                        }
                        _ => {}
                    }

                    cur_token = Token::Word;
                    token_start = i;
                }
                '`' => {
                    cur_token = Token::Name;
                    token_start = i + 1;
                    quote = '`';
                }
                '\'' | '"' => {
                    cur_token = Token::String;
                    token_start = i + 1;
                    quote = *b as char
                }
                '0'...'9' => {
                    match *b as char {
                        '0' => {
                            if i + 1 < len {
                                let next_b = bytes[i + 1] as char;
                                if next_b == 'x' || next_b == 'X' {
                                    cur_token = Token::Hex;
                                    token_start = i;
                                    skip = 1;
                                }
                            }

                            continue;
                        }
                        _ => {}
                    }

                    cur_token = Token::Number;
                    token_start = i;
                }
                '.' | '=' | ';' | '(' | ')' | '?' | '^' | '&' | '|' | '/' | '*' | '+' | '-'
                | ':' | '~' | '<' | '>' | '!' | '%' | ',' => {
                    match *b as char {
                        '(' => p += 1,
                        ')' => p -= 1,
                        _ => {}
                    }
                    cur_token = Token::Symbol;
                    push_token!();
                }
                c if c.is_whitespace() => {}
                _ => {
                    println!("Unexpected byte `{}` at pos {}", *b as char, i);
                    break;
                }
            },
            _ => {}
        }

        if len == i + 1 {
            token_end = i;
            push_token!();
        }
    }
    let end = PreciseTime::now();

    s.push_str("</code></pre>");

    println!("{}", s);
    println!("Took {}", start.to(end));

    fs::write(
        "/home/brian/rust/src/github.com/BrianLeishman/mysql-format/debug.html",
        s,
    ).expect("Unable to write file");
}
