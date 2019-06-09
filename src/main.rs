extern crate actix_web;
extern crate phf;
extern crate time;

use std::fs;
use time::PreciseTime;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn is_function(word: &str) -> bool {
    FUNCTIONS.contains(word)
}
pub fn is_word(word: &str) -> bool {
    KEYWORDS.contains(word)
}

#[derive(PartialEq)]
enum Token {
    Unset,
    Word,
    Name,
    Symbol,
    Number,
    String,
    Hex,
    HexString,
    Function,
    System,
    Variable,
    Placeholder,
}

fn mysql_format2(mysql: &str) -> String {
    let mysql = mysql.trim();
    let bs = mysql.as_bytes();
    let lower = mysql.to_lowercase();
    let len = bs.len();
    if len == 0 {
        return String::default();
    }
    let mut s = String::with_capacity(len * 10);
    let mut i = 0usize;
    let mut j = 0usize;
    let mut start: usize;
    let mut prev_token = Token::Unset;
    let mut end_tag: &str = "";
    // let mut start_tag: &str = "";
    let mut prev_str: &str = "";
    let mut p = 0;
    let mut breakpoint_p = 0;
    let max_line_len = 80;
    let mut cur_line_len = 0;
    let mut last_breakpoint = 0;
    let mut len_after_breakpoint = 0;
    let newline =
        "\n                                                                                "; // ( ͡° ͜ʖ ͡°)
    let insert = (len >= 6 && &lower[..6] == "insert") || (len >= 7 && &lower[..7] == "replace");
    let mut update = len >= 6 && &lower[..6] == "update";
    let mut values = false;

    while i < len {
        macro_rules! next {
            () => {
                i += 1
            };
            ($n:expr) => {
                i += $n
            };
        }

        macro_rules! prev {
            () => {
                i -= 1
            };
        }

        // macro_rules! push {
        // 	($c:expr) => {
        // 		s.push($c);
        // 	};
        // }

        macro_rules! l_push {
            ($c:expr) => {{
                s.push($c);
                cur_line_len += 1;
                len_after_breakpoint += 1;

                if cur_line_len > max_line_len {
                    // s.push_str(&format!("<u>{{{{yes: cur_line_len: {}}}}}</u>", cur_line_len));
                    push_breakpoint_newline!();
                } else {
                    // s.push_str(&format!("<u>{{{{not: cur_line_len: {}}}}}</u>", cur_line_len));
                }
            }};
        }

        macro_rules! push_esc {
            ($c:expr) => {
                match $c {
                    '<' => push_str!("&lt;"),
                    '>' => push_str!("&gt;"),
                    '&' => push_str!("&amp;"),
                    '\n' => push_str!("\\n"),
                    _ => l_push!($c),
                }
            };
        }

        macro_rules! push_str {
            ($s:expr) => {
                s.push_str($s);
            };
        }

        macro_rules! l_push_str {
			($s:expr) => {
				s.push_str($s);
				cur_line_len += $s.len();

                if cur_line_len > max_line_len {
                    // push_str!(&format!("<u>{{{{yes: cur_line_len: {}, last_breakpoint: {}, len_after_breakpoint: {}}}}}</u>", cur_line_len, last_breakpoint, len_after_breakpoint));
                    push_breakpoint_newline!();
                } else {
                    // push_str!(&format!("<u>{{{{not: cur_line_len: {}, last_breakpoint: {}, len_after_breakpoint: {}}}}}</u>", cur_line_len, last_breakpoint, len_after_breakpoint));
                }
			};
		}

        macro_rules! push_str_esc {
            ($s:expr) => {
                for c in $s.chars() {
                    push_esc!(c);
                }
            };
        }

        macro_rules! push_newline {
            () => {{
                s.push_str(&newline[..=p * 2]);
                cur_line_len = p * 2;
                len_after_breakpoint = 0;
                breakpoint_p = p;
            }};
        }

        macro_rules! push_breakpoint_newline {
            () => {
                if last_breakpoint != 0 {
                    s.insert_str(last_breakpoint, &newline[..=breakpoint_p * 2]);
                    cur_line_len = len_after_breakpoint;
                    last_breakpoint = 0;
                    len_after_breakpoint = 0;
                    breakpoint_p = p;
                }
            };
        }

        macro_rules! push_token {
            ($t:ident, $s:expr, $e:expr) => {
                if prev_token != Token::$t {
                    push_str!($s);
                }

                prev_token = Token::$t;

                // start_tag = $s;
                end_tag = $e;
            };
        }

        macro_rules! prep_token {
            ($t:ident) => {
                match prev_token {
                    Token::$t => {}
                    _ => push_str!(end_tag),
                }
            };
            ($t:ident, $s:expr, $e:expr) => {
                prep_token!($t);

                push_token!($t, $s, $e);
            };
            ($t:ident, $s:expr, $e:expr, $($p:ident),+) => {
                prep_token!($t);

                match prev_token {
                    $(Token::$p)|+ => l_push!(' '),
                    _ => {}
                }

                push_token!($t, $s, $e);
            };
        }

        macro_rules! consume_until_esc {
            ($mand_1:expr) => {
                while i < len {
                    match bs[i] {
                        $mand_1 => {
                            break;
                        }
                        b'\\' => next!(2),
                        _ => next!(),
                    }
                }
                prev!();
                if i >= len {
                    i = len - 1
                }
            };
        }

        macro_rules! consume_all_of {
            ($($p:pat)|+) => {
                next!();
                while i < len {
                    match bs[i] {
                        $($p)|* => next!(),
                        _ => {
                            prev!();
                            break;
                        },
                    }
                }
                if i >= len {
                    i = len - 1
                }
            };
        }

        macro_rules! next_non_space {
            () => {{
                j = i + 1;
                while j < len {
                    match bs[j] {
                        b' ' | b'\n' | b'\r' | b'\t' => j += 1,
                        _ => {
                            break;
                        }
                    }
                }
                bs[j]
            }};
        }

        macro_rules! push_token_name {
            () => {
                prep_token!(Name, "<span style=\"color:#674818\">", "</span>", Name);
                l_push!('`');
                push_str_esc!(&mysql[start..=i]);
                l_push!('`');
            };
        }

        macro_rules! push_token_string {
            () => {
                push_token_string!(&mysql[start..=i]);
            };
            ($st:expr) => {{
                let l = &mysql[start..=i].len();
                // let long = l > &(max_line_len - p * 2);

                let t_start = "<span style=\"color:#009688\">";
                let t_end = "</span>";

                // if long {
                //     push_newline!();
                //     push_token_function!("concat");
                //     push_token_symbol!(b'(');

                //     // push_breakpoint_newline!();

                //     let str_len = max_line_len - cur_line_len - 10;
                //     // let padding = cur_line_len + 9 - p*2;
                //     let len = $st.len();
                //     let mut last_escaped = false;
                //     // let mut j = 0;
                //     for mut i in &mut (0..len).step_by(str_len) {
                //         prep_token!(String, t_start, t_end);
                //         l_push!('\'');
                //         if i + str_len + 1 < len {
                //             if last_escaped {
                //                 last_escaped = false;
                //             // j = i - 1;
                //             } else {
                //                 // j = i;
                //             }
                //             if $st.as_bytes()[i + str_len - 1] == b'\\' {
                //                 push_str_esc!($st[i..i + str_len - 1]);
                //                 i -= 1;
                //                 last_escaped = true;
                //             } else {
                //                 push_str_esc!($st[i..i + str_len]);
                //             }
                //         } else {
                //             push_str_esc!($st[i..]);
                //         }
                //         l_push!('\'');

                //         if i + str_len < len {
                //             if last_escaped {
                //                 l_push!(' ');
                //             }
                //             push_token_symbol!(b',');
                //             push_newline!();
                //             l_push_str!(&newline[1..8]);
                //         }
                //     }

                //     push_token_symbol!(b')');
                // } else {
                prep_token!(String, t_start, t_end);
                l_push!('\'');
                push_str_esc!($st);
                l_push!('\'');
                // }
            }};
        }

        macro_rules! push_token_number {
            () => {
                prep_token!(
                    Number,
                    "<span style=\"color:#b71c1c\">",
                    "</span>",
                    Number,
                    Word,
                    Function,
                    System,
                    Variable,
                    Hex,
                    Placeholder
                );
                push_str!(&mysql[start..=i]);
            };
        }

        macro_rules! push_token_symbol {
            () => {
                push_token_symbol!(bs[i]);
            };
            ($sym:expr) => {
                prep_token!(Symbol, "<b>", "</b>");
                push_esc!($sym as char);

                match $sym {
                    b',' => {
                        if update && p == 0 {
                            push_newline!();
                        } else {
                            last_breakpoint = s.len();
                            len_after_breakpoint = 0;
                            breakpoint_p = p;
                        }
                    }
                    b'=' | b'+' | b'-' | b'*' | b'/' | b')' => {
                        last_breakpoint = s.len();
                        len_after_breakpoint = 0;
                        breakpoint_p = p + 1;
                    }
                    _ => {}
                }
            };
        }

        macro_rules! push_token_placeholder {
            () => {
                push_token_placeholder!(bs[i]);
            };
            ($sym:expr) => {
                prep_token!(
                    Placeholder,
                    "<b>",
                    "</b>",
                    Number,
                    Word,
                    Function,
                    System,
                    Variable,
                    Hex,
                    Placeholder
                );
                l_push!($sym as char);
            };
        }

        macro_rules! push_token_word {
            () => {
                push_token_word!(&lower[start..=i]);
            };
            ($word:expr) => {
                prep_token!(
                    Word,
                    "<b style=\"color:#2962FF\">",
                    "</b>",
                    Number,
                    Word,
                    Function,
                    System,
                    Variable,
                    Hex,
                    Placeholder
                );

                match $word {
                    "from" | "where" | "and" | "or" | "order" | "group" | "having" | "limit"
                    | "straight_join" | "cross" | "natural" | "union" | "case" | "set" => {
                        push_newline!()
                    }
                    "left" | "right" => match prev_token {
                        Token::Word => match prev_str {
                            "natural" => {}
                            _ => push_newline!(),
                        },
                        _ => push_newline!(),
                    },
                    "join" => match prev_token {
                        Token::Word => match prev_str {
                            "inner" | "cross" | "outer" | "left" | "right" | "natural" => {}
                            _ => push_newline!(),
                        },
                        _ => push_newline!(),
                    },
                    _ => {}
                }

                if insert && $word == "on" {
                    push_newline!();
                }

                l_push_str!($word);

                if insert {
                    if !values && $word == "values" {
                        values = true;
                        push_newline!();
                    }
                    if $word == "update" {
                        update = true;
                        values = true;
                        push_newline!();
                    }
                }

                prev_str = $word;

                if !insert {
                    match $word {
                        "on" => {
                            last_breakpoint = s.len();
                            len_after_breakpoint = 0;
                            breakpoint_p = p + 1;
                        }
                        _ => {}
                    }
                }
            };
        }

        macro_rules! push_token_function {
            () => {
                push_token_function!(&lower[start..=i]);
            };
            ($func:expr) => {
                prep_token!(
                    Function,
                    "<span style=\"color:#be03de\">",
                    "</span>",
                    Number,
                    Word,
                    Function,
                    System,
                    Variable,
                    Hex,
                    Placeholder
                );
                l_push_str!($func);
            };
        }

        macro_rules! push_token_system {
            () => {
                prep_token!(
                    System,
                    "<i style=\"color:#00ACC1\">",
                    "</i>",
                    Number,
                    Word,
                    Function,
                    System,
                    Variable,
                    Hex,
                    Placeholder
                );
                l_push_str!(&mysql[start..=i]);
            };
        }

        macro_rules! push_token_variable {
            () => {
                prep_token!(
                    Variable,
                    "<span style=\"color:#546E7A\">",
                    "</span>",
                    Number,
                    Word,
                    Function,
                    System,
                    Variable,
                    Hex,
                    Placeholder
                );
                l_push_str!(&mysql[start..=i]);
            };
        }

        macro_rules! push_token_hex {
            ($h:ident) => {
                prep_token!(
                    $h,
                    "<span style=\"color:#4A148C;background-color:#0000001a\">",
                    "</span>",
                    Number,
                    Word,
                    Function,
                    System,
                    Variable,
                    Hex,
                    Placeholder
                );
                if bs[start] == b'0' {
                    l_push_str!("0x");
                    l_push_str!(&mysql[start + 2..=i]);
                } else if i + 1 - start > 3 {
                    l_push_str!("0x");
                    l_push_str!(&mysql[start + 2..i]);
                } else {
                    l_push_str!("''");
                }
            };
        }

        match bs[i] {
            b'`' => {
                next!();
                start = i;
                consume_until_esc! {b'`'};
                push_token_name!();
                next!();
            }
            b'\'' => {
                next!();
                start = i;
                consume_until_esc! {b'\''};
                push_token_string!();
                next!();
            }
            b'"' => {
                next!();
                start = i;
                consume_until_esc! {b'"'};
                push_token_string!();
                next!();
            }
            b'0'...b'9' => {
                start = i;
                if i + 1 < len && (bs[i + 1] == b'x' || bs[i + 1] == b'X') {
                    next!();
                    consume_all_of!(b'0' ... b'9' | b'a' ... b'f' | b'A' ... b'F');
                    push_token_hex!(Hex);
                } else {
                    consume_all_of!(b'0' ... b'9' | b'.');
                    push_token_number!();
                }
            }
            b'.' | b'+' | b'-' => {
                if i + 1 < len {
                    match bs[i + 1] {
                        b'0'...b'9' | b'.' | b'+' | b'-' => {
                            start = i;
                            consume_all_of!(b'0' ... b'9' | b'.');
                            push_token_number!();
                        }
                        _ => {
                            push_token_symbol!();
                        }
                    }
                } else {
                    push_token_symbol!();
                }
            }
            b'=' | b';' | b'(' | b')' | b'^' | b'&' | b'|' | b'/' | b'*' | b':' | b'~'
            | b'<' | b'>' | b'!' | b'%' | b',' => {
                match bs[i] {
                    b'(' => {
                        p += 1;
                    }
                    b')' => p -= 1,
                    _ => {}
                }
                push_token_symbol!();
            }
            b'?' => {
                push_token_placeholder!();
            }
            b'A'...b'Z' | b'a'...b'z' | b'$' | b'_' => {
                start = i;
                if i + 1 < len && (bs[i] == b'x' || bs[i] == b'X') && bs[i + 1] == b'\'' {
                    next!(2);
                    consume_until_esc!(b'\'');
                    next!();
                    push_token_hex!(HexString);
                } else {
                    consume_all_of!(b'0' ... b'9' | b'A'...b'Z' | b'a'...b'z' | b'$' | b'_');
                    if i + 1 < len
                        && next_non_space!() == b'('
                        && (!insert || values || &lower[start..=i] != "values")
                        && is_function(&lower[start..=i])
                    {
                        push_token_function!();
                    } else if is_word(&lower[start..=i]) {
                        push_token_word!();
                    } else {
                        push_token_name!();
                    }
                }
            }
            b'@' => {
                start = i;
                let mut sys = false;
                if i + 1 < len && bs[i + 1] == b'@' {
                    sys = true;
                    next!();
                }
                consume_all_of!(b'0' ... b'9' | b'A'...b'Z' | b'a'...b'z' | b'$' | b'_');
                if sys {
                    push_token_system!();
                } else {
                    push_token_variable!();
                }
            }
            _ => {}
        }

        next!();
        if i >= len {
            prep_token!(Unset);
        }
    }

    return s;
}

use actix_web::{http, server, App, HttpRequest, HttpResponse};

fn index(req: &HttpRequest) -> HttpResponse {
    let q = req.query().get("q").unwrap().to_owned();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(mysql_format2(&q))
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.method(http::Method::GET).f(index)))
        .bind("127.0.0.1:48627")
        .unwrap()
        .run();
}
