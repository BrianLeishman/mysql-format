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
select`orderchildlineitems`.`Quantity`,`orderchildlineitems`.`Price`,
`orderchildlineitems`.`OrderLineItemID`,`ifempty`(
    `networkchildlineitems`.`PluralName`,`networkchildlineitems`.`Name`,
    `childlineitems`.`PluralName`,`childlineitems`.`Name`,
    `orderchildlineitems`.`Child`)`PluralName`,`ifempty`(
    `networkchildlineitems`.`Name`,`networkchildlineitems`.`PluralName`,
    `childlineitems`.`Name`,`childlineitems`.`PluralName`,
    `orderchildlineitems`.`Child`)`Name`,`ifempty`(
    `networkchildlineitemoptions`.`PluralName`,`networkchildlineitemoptions`.`Name`,
    `childlineitemoptions`.`PluralName`,`childlineitemoptions`.`Name`,
    `orderchildlineitems`.`Option`)`OptionPluralName`,`ifempty`(
    `networkchildlineitemoptions`.`Name`,`networkchildlineitemoptions`.`PluralName`,
    `childlineitemoptions`.`Name`,`childlineitemoptions`.`PluralName`,
    `orderchildlineitems`.`Option`)`OptionName`,`orderchildlineitems`.`Description`
from`orderchildlineitems`
left join`childlineitems`using(`ChildLineItemID`)
left join`networkchildlineitems`on`networkchildlineitems`.`NetworkID`=`unb64u`(
    'FUGkiMh0GfI')
and`networkchildlineitems`.`ChildLineItemID`=`orderchildlineitems`.`ChildLineItemID`
and`networkchildlineitems`.`__Active`<>0
left join`childlineitemoptions`on`childlineitemoptions`.`ChildLineItemOptionID`=`orderchildlineitems`.`ChildLineItemOptionID`
and`childlineitemoptions`.`ChildLineItemID`=`childlineitems`.`ChildLineItemID`
left join`networkchildlineitemoptions`on`networkchildlineitemoptions`.`NetworkID`=`unb64u`(
    'FUGkiMh0GfI')
and`networkchildlineitemoptions`.`ChildLineItemOptionID`=`childlineitemoptions`.`ChildLineItemOptionID`
and`networkchildlineitemoptions`.`__Active`<>0
where`orderchildlineitems`.`OrderLineItemID`in(`unb64u`('FWXgjBCaZpA'))
and(`networkchildlineitems`.`Enabled`<>0
    or`childlineitems`.`ChildLineItemID`is null
    or`orderchildlineitems`.`_Total`<>0)
and`orderchildlineitems`.`__Active`<>0
order by`orderchildlineitems`.`SortOrder`,
`orderchildlineitems`.`OrderChildLineItemID`;
"##;

const _MYSQL5: &str = r##"
insert into`sentemailbodies`(`SentEmailBodyID`,`SentEmailID`,`Body`,`Type`)
values
(`buuid`(),`unuuid`('a31939d8-e52f-11e8-a0c2-0e7b3ef972a6'),
    '<table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" style=\"background-color:#f1f0f0\" width=\"100%\"> <tbody> <tr> <td> <table align=\"center\" bgcolor=\"#FFFFFF\" border=\"0\" cellpadding=\"0\" cellspacing=\"0\" style=\"margin-top:60px;\" width=\"660\"> <tbody> <tr> <td align=\"left\" style=\"background-color:#f9f9f9\" valign=\"top\"> <table border=\"0\" cellpadding=\"0\" cellspacing=\"0\" style=\"font-size:14pt;background-color:#fefefe\" width=\"660\"> <tbody> <tr> <td style=\"text-align: left; vertical-align: middle; padding: 20px;\"><a href=\"https://signaturecoins.com#se=fa1ced5bd3da72c5454a928495c2cd0bc7235e7cdffa408eaf9be9c8f9a0e1b1\"><img alt=\"Signature Coins\" src=\"https://i.signaturecoins.com/i/BBL/A.png?1\" style=\"width: 320px; height: 35px;\" /></a></td> <td style=\"text-align: center; vertical-align: middle;padding: 20px;\"><a href=\"tel:1 800-953-3607\"><img alt=\"\" src=\"https://i.signaturecoins.com/i/BAj/C.png\" style=\"width: 33px; height: 33px;\" /></a></td> <td style=\"text-align: center; vertical-align: middle;padding: 20px;\"><a href=\"mailto:info@signaturecoins.com\"><img alt=\"\" src=\"https://i.signaturecoins.com/i/BA3/A.png\" style=\"width: 34px; height: 27px;\" /></a></td> </tr> </tbody> </table> </td> </tr> <tr> <td align=\"left\" height=\"2\" style=\"line-height:2px\" valign=\"top\">&nbsp;</td> </tr> <tr> <td bgcolor=\"#FFFFFF\" style=\"background-color:#ffffff;padding:20px 30px;font-size:14px;line-height:1.5em;font-family:Arial,Helvetica,sans-serif;color:#000000\"> <p style=\"margin:16px 0;font-family:Arial,\'Times New Roman\',Times,serif;font-size:16px;line-height:1.5em;color:#000000\">Hello <span style=\"font-family: HelveticaNeue-Light, &quot;Helvetica Neue Light&quot;, &quot;Helvetica Neue&quot;, Helvetica, Arial, sans-serif; font-size: 16px; background-color: rgb(255, 255, 255);\">Adam,</span></p> <p style=\"margin:16px 0;font-family:Arial,\'Times New Roman\',Times,serif;font-size:16px;line-height:1.5em;color:#000000\"><p>Thank you for your purchase! Your order number is 388721. As soon as your order ships, the tracking number will be sent to you.</p> <table border=\"0\" cellpadding=\"5\" cellspacing=\"1\" style=\"width:100%;border-collapse: collapse;font-family:\'HelveticaNeue-Light\',\'Helvetica Neue Light\',\'Helvetica Neue\',Helvetica,Arial,sans-serif;font-size:16px;line-height:24px;font-weight:300\"> <tbody> <tr> <td style=\"vertical-align:top;\"><strong>Shipped to:</strong> <address>Adam Crenshaw<br /> Custom Canine Unlimited<br /> 1316 Welborn Rd.<br /> Gillsville, GA 30543<br /> United States<br /> (470) 577-4191<br /> acrenshaw@customcanineunlimited.com</address> </td> <td style=\"vertical-align:top;text-align:right;\"><span height=\"75\" style=\"height:75px;max-height:75px;\"><img alt=\"proof-5.jpg\" height=\"75\" src=\"https://i.signaturecoins.com/proofs/FWNphnVqkzQ-proof-5/thumb.png\" style=\"height:75px;max-height:75px;margin-right:10px;margin-bottom: 5px;\" title=\"proof-5.jpg\" /> </span></td> </tr> </tbody> </table> <table border=\"0\" cellpadding=\"5\" cellspacing=\"1\" style=\"width:100%;border-collapse: collapse;font-family:\'HelveticaNeue-Light\',\'Helvetica Neue Light\',\'Helvetica Neue\',Helvetica,Arial,sans-serif;font-size:16px;line-height:24px;font-weight:300\"> <thead> <tr style=\"vertical-align: bottom;border-bottom: 2px solid #ddd;\"> <th style=\"text-align:left;\">Item</th> <th style=\"text-align:right;\">Quantity</th> <th style=\"text-align:right;\">Unit Price</th> </tr> </thead> <tbody> <tr style=\"vertical-align: top;border-top: 1px solid #ddd;background-color:#f9f9f9;\"> <td><b>1.75&#34; Color On Both Sides Challenge Coins</b></td> <td style=\"text-align:right;\">100</td> <td style=\"text-align:right;\">$3.85</td> </tr><tr style=\"vertical-align: top;border-top: 1px solid #ddd;background-color:#fff;\"> <td style=\"padding-left:1em;\">Edge: <b>Standard</b></td> <td style=\"text-align:right;\">100</td> <td style=\"text-align:right;\">$0.00</td> </tr><tr style=\"vertical-align: top;border-top: 1px solid #ddd;background-color:#fff;\"> <td style=\"padding-left:1em;\">Mold Fee: <b>New One-Sided Mold</b></td> <td style=\"text-align:right;\">2</td> <td style=\"text-align:right;\">$62.50</td> </tr><tr style=\"vertical-align: top;border-top: 1px solid #ddd;background-color:#fff;\"> <td style=\"padding-left:1em;\">Packaging: <b>Plastic Coin Envelopes</b></td> <td style=\"text-align:right;\">100</td> <td style=\"text-align:right;\">$0.00</td> </tr><tr style=\"vertical-align: top;border-top: 1px solid #ddd;background-color:#fff;\"> <td style=\"padding-left:1em;\">Plating: <b>Silver</b></td> <td style=\"text-align:right;\">100</td> <td style=\"text-align:right;\">$0.00</td> </tr> <tr style=\"vertical-align: top;border-top: 2px solid #ddd;\"> <td>&nbsp;</td> <td style=\"text-align:right;\"><b>Total</b></td> <td style=\"text-align:right;\"><b>$510.00</b></td> </tr> </tbody> </table> </p> <p style=\"font-family:Arial,\'Times New Roman\',Times,serif;font-size:16px;line-height:1.5em;color:#000000\"><span style=\"line-height: 1.5em; \">If there&rsquo;s anything we can do to help, please give us a call at&nbsp;</span>1 800-953-3607<span style=\"line-height: 1.5em;\">&nbsp;<span style=\"font-family: Arial, &quot;Times New Roman&quot;, Times, serif; font-size: 16px; background-color: rgb(255, 255, 255);\">from 9:00&nbsp;</span><span style=\"font-family: Arial, &quot;Times New Roman&quot;, Times, serif; font-size: 16px; background-color: rgb(255, 255, 255);\">&ndash;</span><span style=\"font-family: Arial, &quot;Times New Roman&quot;, Times, serif; font-size: 16px; background-color: rgb(255, 255, 255);\">&nbsp;19:00 EST&nbsp;</span>or email anytime!</span></p> <p style=\"font-family: Arial, \'Times New Roman\', Times, serif; font-size: 16px; line-height: 1.5em;color:#000000\">Best Regards,</p> <p style=\"font-family: Arial, \'Times New Roman\', Times, serif; font-size: 16px; line-height: 1.5em;color:#000000\"></p> </td> </tr> <tr> <td bgcolor=\"#f1f0f0\" style=\"padding:15px 0 0 0\"> <table align=\"center\" border=\"0\" cellpadding=\"0\" cellspacing=\"0\"> <tbody> <tr> <td align=\"center\" style=\"text-align:center;padding-top:0px;padding-bottom:15px;font-family:Arial,Helvetica,sans-serif;color:#8f8f8f;font-size:18px;font-style:italic;text-decoration:none;line-height:22px\" valign=\"top\">With 18 Years of Experience, You Can Trust Signature Coins</td> </tr> <tr> <td align=\"left\" style=\"text-align:center;padding-bottom:7px\" valign=\"top\"><a href=\"https://www.facebook.com/signaturepins\" style=\"font-family:Arial,sans-serif;color:#676767;font-size:13px;text-decoration:none\" target=\"_blank\"><strong>Facebook</strong></a>&nbsp;&nbsp;<strong style=\"font-family: Arial, sans-serif; color: rgb(103, 103, 103); font-size: 13px; text-decoration: none;\"><a href=\"https://www.twitter.com/signaturepins\" style=\"font-family:Arial,sans-serif;color:#676767;font-size:13px;text-decoration:none\" target=\"_blank\">Twitter</a></strong>&nbsp;&nbsp;<strong style=\"text-align: center; font-family: Arial, sans-serif; color: rgb(103, 103, 103); font-size: 13px; text-decoration: none; background-color: rgb(241, 240, 240);\"><a href=\"https://plus.google.com/109092569940405123143/posts\" style=\"text-align: center; font-family: Arial, sans-serif; color: rgb(103, 103, 103); font-size: 13px; text-decoration: none; background-color: rgb(241, 240, 240);\" target=\"_blank\">Google+</a>&nbsp;&nbsp;</strong><strong style=\"text-align: center; font-family: Arial, sans-serif; color: rgb(103, 103, 103); font-size: 13px; background-color: rgb(241, 240, 240);\"><a href=\"http://www.instagram.com/signaturepins/\" style=\"color: rgb(103, 103, 103); text-decoration: none;\" target=\"_blank\">Instagram</a></strong><strong style=\"text-align: center; font-family: Arial, sans-serif; color: rgb(103, 103, 103); font-size: 13px; text-decoration: none; background-color: rgb(241, 240, 240);\">&nbsp;</strong>&nbsp;<a href=\"https://www.pinterest.com/signaturepins/\" style=\"font-family:Arial,sans-serif;color:#676767;font-size:13px;text-decoration:none\" target=\"_blank\"><strong>Pinterest</strong></a><br /> &nbsp;</td> </tr> <tr> <td align=\"left\" style=\"font-family:Arial,Helvetica,sans-serif;color:#9c9c9c;font-size:12px;text-decoration:none;text-align:center;padding-bottom:10px\" valign=\"top\">To ensure delivery, add us to your address book: <a href=\"mailto:info@signaturecoins.com\" style=\"color:#474747;text-decoration:none;\">info@signaturecoins.com</a></td> </tr> </tbody> </table> <table align=\"center\" border=\"0\" cellpadding=\"0\" cellspacing=\"0\" width=\"570\"> <tbody> <tr> <td style=\"font-family: Arial, Helvetica, sans-serif; color: rgb(156, 156, 156); font-size: 11px; text-decoration: none; padding-bottom: 30px; padding-top: 10px; line-height: 14px; text-align: center;\" valign=\"top\">&copy; 2018 SIGNATURE PROMOTIONAL GROUP LLC. All Rights Reserved.<br /> Signature Coins, <span>16877 E Colonial Dr, #313 Orlando, Florida 32820</span><br /> <span style=\"color: rgb(156, 156, 156); font-family: Arial, Helvetica, sans-serif; font-size: 11px; line-height: 14px; background-color: rgb(241, 240, 240);\">Signature Coins</span>&nbsp;and the <span style=\"color: rgb(156, 156, 156); font-family: Arial, Helvetica, sans-serif; font-size: 11px; line-height: 14px; background-color: rgb(241, 240, 240);\">Signature Coins</span> logo are registered trademarks or trademarks of<br /> <span style=\"color: rgb(156, 156, 156); font-family: Arial, Helvetica, sans-serif; font-size: 11px; line-height: 14px; background-color: rgb(241, 240, 240);\">SIGNATURE PROMOTIONAL GROUP LLC</span>&nbsp;in the United States and other countries.</td> </tr> </tbody> </table> </td> </tr> </tbody> </table> </td> </tr> </tbody> </table> <div>&nbsp;</div> <div>&nbsp;</div> <img src=\"https://signaturecoins.com/img/fa1ced5bd3da72c5454a928495c2cd0bc7235e7cdffa408eaf9be9c8f9a0e1b1.gif\" alt=\"\" />',
    'text/html'),
(`buuid`(),`unuuid`('a31939d8-e52f-11e8-a0c2-0e7b3ef972a6'),`unb64u`(
        'WzFdClsyXQpbM10KCsKgCgpIZWxsbyBBZGFtLAoKVGhhbmsgeW91IGZvciB5b3VyIHB1cmNoYXNlISBZb3VyIG9yZGVyIG51bWJlciBpcyAzODg3MjEuIEFzIHNvb24gYXMKeW91ciBvcmRlciBzaGlwcywgdGhlIHRyYWNraW5nIG51bWJlciB3aWxsIGJlIHNlbnQgdG8geW91LgoKU0hJUFBFRCBUTzogQWRhbSBDcmVuc2hhdwpDdXN0b20gQ2FuaW5lIFVubGltaXRlZAoxMzE2IFdlbGJvcm4gUmQuCkdpbGxzdmlsbGUsIEdBIDMwNTQzClVuaXRlZCBTdGF0ZXMKKDQ3MCkgNTc3LTQxOTEKYWNyZW5zaGF3QGN1c3RvbWNhbmluZXVubGltaXRlZC5jb20KCklURU0KUVVBTlRJVFkKVU5JVCBQUklDRQoKMS43NSYjMzQ7IENPTE9SIE9OIEJPVEggU0lERVMgQ0hBTExFTkdFIENPSU5TCjEwMAokMy44NQoKRWRnZTogU1RBTkRBUkQKMTAwCiQwLjAwCgpNb2xkIEZlZTogTkVXIE9ORS1TSURFRCBNT0xECjIKJDYyLjUwCgpQYWNrYWdpbmc6IFBMQVNUSUMgQ09JTiBFTlZFTE9QRVMKMTAwCiQwLjAwCgpQbGF0aW5nOiBTSUxWRVIKMTAwCiQwLjAwCgrCoApUT1RBTAokNTEwLjAwCgpJZiB0aGVyZT9zIGFueXRoaW5nIHdlIGNhbiBkbyB0byBoZWxwLCBwbGVhc2UgZ2l2ZSB1cyBhIGNhbGwgYXTCoDEKODAwLTk1My0zNjA3wqBmcm9tIDk6MDDCoD_CoDE5OjAwIEVTVMKgb3IgZW1haWwgYW55dGltZSEKCkJlc3QgUmVnYXJkcywKCldpdGggMTggWWVhcnMgb2YgRXhwZXJpZW5jZSwgWW91IENhbiBUcnVzdCBTaWduYXR1cmUgQ29pbnMKCkZBQ0VCT09LIFs0XcKgwqBUV0lUVEVSIFs1XcKgwqBHT09HTEUrIFs2XcKgwqBJTlNUQUdSQU0KWzddwqDCoFBJTlRFUkVTVCBbOF0KwqAKClRvIGVuc3VyZSBkZWxpdmVyeSwgYWRkIHVzIHRvIHlvdXIgYWRkcmVzcyBib29rOgppbmZvQHNpZ25hdHVyZWNvaW5zLmNvbSBbOV0KCsKpIDIwMTggU0lHTkFUVVJFIFBST01PVElPTkFMIEdST1VQIExMQy4gQWxsIFJpZ2h0cyBSZXNlcnZlZC4KU2lnbmF0dXJlIENvaW5zLCAxNjg3NyBFIENvbG9uaWFsIERyLCAjMzEzIE9ybGFuZG8sIEZsb3JpZGEgMzI4MjAKU2lnbmF0dXJlIENvaW5zwqBhbmQgdGhlIFNpZ25hdHVyZSBDb2lucyBsb2dvIGFyZSByZWdpc3RlcmVkCnRyYWRlbWFya3Mgb3IgdHJhZGVtYXJrcyBvZgpTSUdOQVRVUkUgUFJPTU9USU9OQUwgR1JPVVAgTExDwqBpbiB0aGUgVW5pdGVkIFN0YXRlcyBhbmQgb3RoZXIKY291bnRyaWVzLgoKwqAgwqAKCkxpbmtzOgotLS0tLS0KWzFdCmh0dHBzOi8vc2lnbmF0dXJlY29pbnMuY29tI3NlPWZhMWNlZDViZDNkYTcyYzU0NTRhOTI4NDk1YzJjZDBiYzcyMzVlN2NkZmZhNDA4ZWFmOWJlOWM4ZjlhMGUxYjEKWzJdIHRlbDoxIDgwMC05NTMtMzYwNwpbM10gbWFpbHRvOmluZm9Ac2lnbmF0dXJlY29pbnMuY29tCls0XSBodHRwczovL3d3dy5mYWNlYm9vay5jb20vc2lnbmF0dXJlcGlucwpbNV0gaHR0cHM6Ly93d3cudHdpdHRlci5jb20vc2lnbmF0dXJlcGlucwpbNl0gaHR0cHM6Ly9wbHVzLmdvb2dsZS5jb20vMTA5MDkyNTY5OTQwNDA1MTIzMTQzL3Bvc3RzCls3XSBodHRwOi8vd3d3Lmluc3RhZ3JhbS5jb20vc2lnbmF0dXJlcGlucy8KWzhdIGh0dHBzOi8vd3d3LnBpbnRlcmVzdC5jb20vc2lnbmF0dXJlcGlucy8KWzldIG1haWx0bzppbmZvQHNpZ25hdHVyZWNvaW5zLmNvbQ'),
    'text/plain');
"##;

fn mysql_format2(mysql: &str) -> String {
	let mysql = mysql.trim();
	let bs = mysql.as_bytes();
	let lower = mysql.to_lowercase();
	let len = bs.len();
	let mut s = String::with_capacity(len * 10);
	s.push_str("<pre>");
	let mut i = 0usize;
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
	let newline = "\n                                                                                "; // ( ͡° ͜ʖ ͡°)
	let insert = &lower[..6] == "insert";
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
				s.push_str(&newline[..=p*2]);
				cur_line_len = p*2;
				len_after_breakpoint = 0;
				breakpoint_p = p;
			}};
		}

        macro_rules! push_breakpoint_newline {
            () => {
                if last_breakpoint != 0 {
					s.insert_str(last_breakpoint, &newline[..=breakpoint_p*2]);
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

                // if cur_line_len > max_line_len {
                //     push_breakpoint_newline!();
                // }
            };
            ($t:ident, $s:expr, $e:expr, $($p:ident),+) => {
                prep_token!($t);

                match prev_token {
                    $(Token::$p)|+ => l_push!(' '),
                    _ => {}
                }

                push_token!($t, $s, $e);

                // if cur_line_len > max_line_len {
                //     push_breakpoint_newline!();
                // } else {
                //     s.push_str(&format!("<u>{{{{cur_line_len: {}}}}}</u>", cur_line_len));
                // }
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
				let long = l > &(max_line_len - p*2);

				let t_start = "<span style=\"color:#009688\">";
				let t_end = "</span>";

				if long {
                    push_newline!();
                    push_token_function!("concat");
					push_token_symbol!(b'(');

                    // push_breakpoint_newline!();

					let str_len = max_line_len - cur_line_len - 10;
                    // let padding = cur_line_len + 9 - p*2;
					let len = $st.len();
                    let mut last_escaped = false;
                    // let mut j = 0;
					for mut i in &mut (0..len).step_by(str_len) {
                        prep_token!(String, t_start, t_end);
						l_push!('\'');
						if i + str_len + 1 < len {
                            if last_escaped {
                                last_escaped = false;
                                // j = i - 1;
                            } else {
                                // j = i;
                            }
                            if $st.as_bytes()[i+str_len-1] == b'\\' {
							    push_str_esc!($st[i..i+str_len-1]);
                                i -= 1;
                                last_escaped = true;
                            } else {
                                push_str_esc!($st[i..i+str_len]);
                            }
						} else {
							push_str_esc!($st[i..]);
						}
						l_push!('\'');

						if i + str_len < len {
                            if last_escaped {
                                l_push!(' ');
                            }
							push_token_symbol!(b',');
							push_newline!();
							l_push_str!(&newline[1..8]);
						}
					}

					push_token_symbol!(b')');
				} else {
                    prep_token!(String, t_start, t_end);
					l_push!('\'');
					push_str_esc!($st);
					l_push!('\'');
				}
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
					Hex
				);
				push_str_esc!(&mysql[start..=i]);
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
						last_breakpoint = s.len();
						len_after_breakpoint = 0;
						breakpoint_p = p;
					},
					b'=' | b'+' | b'-' | b'*' | b'/' | b')' => {
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
					Hex
                );

				match $word {
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

				if insert && !values && $word == "values" {
					values = true;
				}

				l_push_str!($word);

				prev_str = $word;

				match $word {
					"on" => {
						last_breakpoint = s.len();
						len_after_breakpoint = 0;
						breakpoint_p = p + 1;
					},
					_ => {}
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
					Hex
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
					b'(' => {
						if insert && values && p == 0 {
							push_newline!();
						}
						p += 1;
					},
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
	let s = mysql_format2(_MYSQL3);
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
