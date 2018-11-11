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
}

const _MYSQL: &str = "sELECT`users`.`UserID`id,now(+6365)
fRoM`users`
JOIN`companies`using(`CompanyID`)
where`users`.`Email`='プログ\\'ラマーズ>'
and`companies`.`NetworkID`=x'1541A488C87419F2'
and`companies`.`NetworkID`=0x1541A488C87419F2
and`companies`.`CompanyID`in(@@AdminCompanyIDs)
and yeet.poop   in('')
and`users`.`__Active`<>0.0 
and @i := -.232
order by`users`.`__Added`desc
limit 1;";

const _MYSQL2: &str = "select`lineitemtypes`.`LineItemTypeID`,ifnull(`lineitemtypes`.`PluralName`,
    `lineitemtypes`.`Name`)`PluralName`,ifnull(`networklineitemtypes`.`PluralName`,
    `networklineitemtypes`.`Name`)`NetworkPluralName`
from`lineitemtypes`
join`lineitems`using(`LineItemID`)
left join`networklineitemtypes`on`networklineitemtypes`.`NetworkID`=`unb64u`(
    'FUGkiMhy53I')
and`networklineitemtypes`.`LineItemTypeID`=`lineitemtypes`.`LineItemTypeID`
where`lineitemtypes`.`LineItemTypeID`=`unb64u`('FUGjIgIAlDI')
and`lineitems`.`__Active`=1 
and`lineitemtypes`.`__Active`=1;";

const _MYSQL3: &str = r##"
insert into`quoterequestimages`(`QuoteRequestImageID`,`QuoteRequestID`,`Path`,
    `Name`,`Files`,`TotalSize`,`Hash`)
values
(`unb64u`('FWYoWfoTqKA'),`unb64u`('FWYoWfabSEA'),
    'quoterequests/FWYoWfabSEA-lamotte-township-fire-dept-brad-white/FWYoWfoTqKA-700-patch-2.wmf',
    '700 Patch 2.wmf',
    '{\n    \"Original\": {\n        \"File\": \"original.wmf\",\n        \"Width\": 612,\n        \"Height\": 792,\n        \"Size\": 474738\n    },\n    \"Thumbnail\": {\n        \"File\": \"thumbnail.jpg\",\n        \"Width\": 116,\n        \"Height\": 150,\n        \"Size\": 2094\n    },\n    \"Raster\": {\n        \"File\": \"raster.png\",\n        \"Width\": 612,\n        \"Height\": 792,\n        \"Size\": 132565\n    }\n}',
    609397,`unb64u`('2ttmmGNN8149wt-GD3G_52IB23QZnCA0blPviRLZsfo')),
(`unb64u`('FWYoWgQReoA'),`unb64u`('FWYoWfabSEA'),
    'quoterequests/FWYoWfabSEA-lamotte-township-fire-dept-brad-white/FWYoWgQReoA-20f86637-f09d-4f81-b8c5-cbc58dda64e5-20patches-20-20no-20bac.jpg',
    '20f86637-f09d-4f81-b8c5-cbc58dda64e5%20Patches%20-%20No%20Backing[1].jpg',
    '{\n    \"Original\": {\n        \"File\": \"original.jpg\",\n        \"Width\": 225,\n        \"Height\": 208,\n        \"Size\": 20127\n    },\n    \"Thumbnail\": {\n        \"File\": \"thumbnail.jpg\",\n        \"Width\": 150,\n        \"Height\": 139,\n        \"Size\": 7805\n    }\n}',
    27932,`unb64u`('vDRd_HEP_JuZavFog0Eio81lPQ5dp3MSpV6qzpVqQZI'));
"##;

const _MYSQL4: &str = r##"
select`childlineitems`.`ChildLineItemID`,
`childlineitemoptions`.`ChildLineItemOptionID`,`childlineitems`.`AutoQuantity`,
`childlineitems`.`LinkedQuantity`
from`childlineitems`
left join`childlineitemoptions`on`childlineitemoptions`.`ChildLineItemID`=`childlineitems`.`ChildLineItemID`
and`childlineitemoptions`.`Default`
and`childlineitemoptions`.`__Active`=1 
where`childlineitems`.`LineItemID`=`unb64u`('FUGjGOogxzI')
and(`childlineitems`.`Default`
    or`childlineitems`.`ChildLineItemID`in(`unb64u`('FUGi9Ht8cAI'),`unb64u`(
            'FUGi9HvNusI'),`unb64u`('FUGi9HvQ4VI')))
and`childlineitems`.`__Active`=1 
order by`natsort_canon`(`childlineitems`.`Name`,'natural'),`natsort_canon`(
    `childlineitemoptions`.`Name`,'natural'),`childlineitems`.`ChildLineItemID`;
"##;

fn mysql_format2(mysql: &str) -> String {
	let bs = mysql.as_bytes();
	let lower = mysql.to_lowercase();
	let len = bs.len();
	let mut s = String::with_capacity(len * 10);
	s.push_str("<pre>");
	let mut i = 0usize;
	let mut start: usize;
	let mut prev_token = Token::Unset;
	let mut end_tag: &str = "";
	let mut prev_str: &str = "";
	let mut p = 0;
	let mut breakpoint_p = 0;
	let max_line_len = 80;
	let mut cur_line_len = 0;
	let mut last_breakpoint = 0;
	let mut len_after_breakpoint = 0;
	let newline = "\n                                                                                "; // ( ͡° ͜ʖ ͡°)

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

		macro_rules! push {
			($c:expr) => {
				s.push($c);
			};
		}

		macro_rules! l_push {
			($c:expr) => {{
				s.push($c);
				cur_line_len += 1;
				len_after_breakpoint += 1;
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
				s.push_str(&newline[..=p*2]);
				cur_line_len = p*2;
				len_after_breakpoint = 0;
				breakpoint_p = p;
			}};
		}

		macro_rules! push_token {
			($t:ident, $s:expr, $e:expr) => {
				if prev_token != Token::$t {
					push_str!($s);
				}

				prev_token = Token::$t;

				end_tag = $e;
			};
		}

		macro_rules! prep_token {
            ($t:ident) => {
                match prev_token {
                    Token::$t => {}
                    _ => push_str!(end_tag),
                }

				if cur_line_len > max_line_len && last_breakpoint != 0 && len_after_breakpoint != 0 {
					s.insert_str(last_breakpoint, &newline[..=breakpoint_p*2]);
					cur_line_len = len_after_breakpoint;
					last_breakpoint = 0;
					len_after_breakpoint = 0;
					breakpoint_p = p;
				}
            };
            ($t:ident, $s:expr, $e:expr) => {
                prep_token!($t);

                push_token!($t, $s, $e);
            };
            ($t:ident, $s:expr, $e:expr, $($p:ident),+) => {
                prep_token!($t);

                match prev_token {
                    $(Token::$p)|+ => push!(' '),
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
            };
        }

		macro_rules! push_token_name {
			() => {
				prep_token!(Name, "<span style=\"color:#795548\">", "</span>", Name);
				l_push!('`');
				push_str_esc!(&mysql[start..=i]);
				l_push!('`');
			};
		}

		macro_rules! push_token_string {
			() => {
				prep_token!(String, "<span style=\"color:#43A047\">", "</span>");
				l_push!('\'');
				push_str_esc!(&mysql[start..=i]);
				l_push!('\'');
			};
		}

		macro_rules! push_token_number {
			() => {
				prep_token!(
					Number,
					"<span style=\"color:#BF360C\">",
					"</span>",
					Number,
					Word,
					Function,
					System,
					Variable,
					Hex
				);
				push_str_esc!(&mysql[start..=i]);
			};
		}

		macro_rules! push_token_symbol {
			() => {
				prep_token!(Symbol, "<b>", "</b>");
				push_esc!(bs[i] as char);

				match bs[i] {
					b',' => {
						last_breakpoint = s.len();
						len_after_breakpoint = 0;
						breakpoint_p = p;
					},
					b'=' => {
						last_breakpoint = s.len();
						len_after_breakpoint = 0;
						breakpoint_p = p + 1;
					},
					_ => {}
				}
			};
		}

		macro_rules! push_token_word {
			() => {
				prep_token!(
					Word,
					"<b style=\"color:#1565C0\">",
					"</b>",
					Number,
					Word,
					Function,
					System,
					Variable,
					Hex
					);

				match &lower[start..=i] {
					"from" | "where" | "and" | "or" | "order" | "group" | "having" | "limit"
					| "straight_join" | "cross" | "natural" | "union" | "case" => push_newline!(),
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

				l_push_str!(&lower[start..=i]);

				prev_str = &lower[start..=i];
			};
		}

		macro_rules! push_token_function {
			() => {
				prep_token!(
					Function,
					"<span style=\"color:#C51162\">",
					"</span>",
					Number,
					Word,
					Function,
					System,
					Variable,
					Hex
				);
				l_push_str!(&lower[start..=i]);
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
					Hex
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
					Hex
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
					Hex
				);
				l_push_str!(&mysql[start..=i]);
			};
		}

		match bs[i] {
			b'`' => {
				next!();
				start = i;
				consume_until_esc!{b'`'};
				push_token_name!();
				next!();
			}
			b'\'' => {
				next!();
				start = i;
				consume_until_esc!{b'\''};
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
			b'=' | b';' | b'(' | b')' | b'?' | b'^' | b'&' | b'|' | b'/' | b'*' | b':' | b'~'
			| b'<' | b'>' | b'!' | b'%' | b',' => {
				match bs[i] {
					b'(' => p += 1,
					b')' => p -= 1,
					_ => {}
				}
				push_token_symbol!();
			}
			b'A'...b'Z' | b'a'...b'z' | b'$' | b'_' => {
				start = i;
				if i + 1 < len && (bs[i] == b'x' || bs[i] == b'X') && bs[i + 1] == b'\'' {
					next!(2);
					consume_until_esc!{b'\''};
					next!();
					push_token_hex!(HexString);
				} else {
					consume_all_of!(b'0' ... b'9' | b'A'...b'Z' | b'a'...b'z' | b'$' | b'_');
					if i + 1 < len && bs[i + 1] == b'(' && is_function(&lower[start..=i]) {
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
		if i == len {
			prep_token!(Unset);
		}
	}

	s.push_str("</pre>");
	return s;
}

fn main() {
	let start = PreciseTime::now();
	let s = mysql_format2(_MYSQL4);
	let end = PreciseTime::now();

	println!("{}", s);
	println!("Took {}", start.to(end));

	fs::write(
		"debug.html",
		format!(
			"{}{}{}",
			"<html><head><style>html,body{background-color:#eff0f1}</style></head><body>",
			s,
			"</body></html>"
		),
	)
	.expect("Unable to write file");
}
