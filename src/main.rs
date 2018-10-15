extern crate htmlescape;
extern crate time;
// #[macro_use]
// extern crate lazy_static;
// #[macro_use]
// extern crate maplit;

use htmlescape::encode_minimal;
use std::fs;
use time::PreciseTime;

mod words_match;
use words_match::is_word;

mod functions_match;
use functions_match::is_function;

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

const _MYSQL: &str = "select`users`.`UserID` `id`,now(+6365)
from`users`
join`companies`using(`CompanyID`)
where`users`.`Email`='<phil@redshift.com>'
and`companies`.`NetworkID`=x'1541A488C87419F2'
and`companies`.`CompanyID`in(@@AdminCompanyIDs)
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

const _MYSQL3: &str = "select count(*)`Count`from(select  1  from((select `qr`.`QuoteRequestID` from`quoterequests` `qr` left join`quoterequests` `_qr`on`_qr`.`UserID`=`qr`.`UserID`and`_qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`_qr`.`__Active`=1 and`_qr`.`QuoteRequestID`>`qr`.`QuoteRequestID`  where(`qr`.`LinkID`in('be0e4926-8e82-11e8-9cfa-12237f224cef','78891094-8e81-11e8-a116-12237f224cef','04711c56-8e81-11e8-9dcf-12237f224cef','2344153a-8e80-11e8-b396-12237f224cef','07bc92ac-8e7f-11e8-b258-12237f224cef','e52c00a6-85e6-11e8-829c-12237f224cef','1a6552e0-85dd-11e8-8349-12237f224cef','84c33270-85dc-11e8-9d64-12237f224cef','ee9876ca-85db-11e8-b36a-12237f224cef','11b53e46-85db-11e8-8221-12237f224cef','6b339b54-807a-11e8-aa89-12237f224cef','14cb5b10-789d-11e8-8e01-12237f224cef','f94b3702-789c-11e8-a1b6-12237f224cef','d65d3582-73fc-11e8-a459-12237f224cef','98d7858c-73fc-11e8-9568-12237f224cef','2f8961d6-73fc-11e8-b1e9-12237f224cef','a559ba56-73ce-11e8-adaa-12237f224cef','88dcd570-73ce-11e8-924c-12237f224cef','77cdaaf4-7011-11e8-872b-12237f224cef','524619ce-7011-11e8-b1bd-12237f224cef','1bef792e-7011-11e8-87c2-12237f224cef','2d5ab7ce-7010-11e8-874a-12237f224cef','6b8c5a44-700f-11e8-b320-12237f224cef','7e93a09c-7001-11e8-a943-12237f224cef','05b04ad6-7001-11e8-afad-12237f224cef','e8b06cfe-7000-11e8-9473-12237f224cef','cb2c2a92-7000-11e8-aa40-12237f224cef','aee830ba-7000-11e8-a2d6-12237f224cef','92d4e5d0-7000-11e8-b010-12237f224cef','64e8e522-7000-11e8-b884-12237f224cef','bf0dc99a-6ffc-11e8-98f9-12237f224cef','b0323ea2-6ffb-11e8-9413-12237f224cef','699e9864-6ffb-11e8-ab4e-12237f224cef','f4f7cb7a-6ffa-11e8-89ea-12237f224cef','e04abc8c-6ffa-11e8-8368-12237f224cef','c6c1d52a-6ffa-11e8-989a-12237f224cef','83b82aae-6ffa-11e8-b587-12237f224cef','910a4634-6ff9-11e8-a5d8-12237f224cef','90ea2f3a-6ff8-11e8-92e0-12237f224cef','c44a6b2a-6ff7-11e8-8491-12237f224cef','743b66bc-6fe7-11e8-8827-12237f224cef','5d6d6f7a-6fe7-11e8-83f4-12237f224cef','435e1058-6fe7-11e8-b16d-12237f224cef','1e7fcb48-6fe5-11e8-957b-12237f224cef','3417a228-6e4b-11e8-a132-12237f224cef'))and(`_qr`.`QuoteRequestID`is null)and(`qr`.`CompanyID`in(from_base64('FUGW1FiCBPI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GyeryI=')))and(`qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`qr`.`__Active`=1)  limit 1000)union(select `qr`.`QuoteRequestID` from`quoterequests` `qr` join`users` `links_u`on`links_u`.`UserID`=`qr`.`UserID`and`links_u`.`LinkID`in('be0e4926-8e82-11e8-9cfa-12237f224cef','78891094-8e81-11e8-a116-12237f224cef','04711c56-8e81-11e8-9dcf-12237f224cef','2344153a-8e80-11e8-b396-12237f224cef','07bc92ac-8e7f-11e8-b258-12237f224cef','e52c00a6-85e6-11e8-829c-12237f224cef','1a6552e0-85dd-11e8-8349-12237f224cef','84c33270-85dc-11e8-9d64-12237f224cef','ee9876ca-85db-11e8-b36a-12237f224cef','11b53e46-85db-11e8-8221-12237f224cef','6b339b54-807a-11e8-aa89-12237f224cef','14cb5b10-789d-11e8-8e01-12237f224cef','f94b3702-789c-11e8-a1b6-12237f224cef','d65d3582-73fc-11e8-a459-12237f224cef','98d7858c-73fc-11e8-9568-12237f224cef','2f8961d6-73fc-11e8-b1e9-12237f224cef','a559ba56-73ce-11e8-adaa-12237f224cef','88dcd570-73ce-11e8-924c-12237f224cef','77cdaaf4-7011-11e8-872b-12237f224cef','524619ce-7011-11e8-b1bd-12237f224cef','1bef792e-7011-11e8-87c2-12237f224cef','2d5ab7ce-7010-11e8-874a-12237f224cef','6b8c5a44-700f-11e8-b320-12237f224cef','7e93a09c-7001-11e8-a943-12237f224cef','05b04ad6-7001-11e8-afad-12237f224cef','e8b06cfe-7000-11e8-9473-12237f224cef','cb2c2a92-7000-11e8-aa40-12237f224cef','aee830ba-7000-11e8-a2d6-12237f224cef','92d4e5d0-7000-11e8-b010-12237f224cef','64e8e522-7000-11e8-b884-12237f224cef','bf0dc99a-6ffc-11e8-98f9-12237f224cef','b0323ea2-6ffb-11e8-9413-12237f224cef','699e9864-6ffb-11e8-ab4e-12237f224cef','f4f7cb7a-6ffa-11e8-89ea-12237f224cef','e04abc8c-6ffa-11e8-8368-12237f224cef','c6c1d52a-6ffa-11e8-989a-12237f224cef','83b82aae-6ffa-11e8-b587-12237f224cef','910a4634-6ff9-11e8-a5d8-12237f224cef','90ea2f3a-6ff8-11e8-92e0-12237f224cef','c44a6b2a-6ff7-11e8-8491-12237f224cef','743b66bc-6fe7-11e8-8827-12237f224cef','5d6d6f7a-6fe7-11e8-83f4-12237f224cef','435e1058-6fe7-11e8-b16d-12237f224cef','1e7fcb48-6fe5-11e8-957b-12237f224cef','3417a228-6e4b-11e8-a132-12237f224cef')and`links_u`.`__Active`=1 left join`quoterequests` `_qr`on`_qr`.`UserID`=`qr`.`UserID`and`_qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`_qr`.`__Active`=1 and`_qr`.`QuoteRequestID`>`qr`.`QuoteRequestID`  where(`_qr`.`QuoteRequestID`is null)and(`qr`.`CompanyID`in(from_base64('FUGW1FiCBPI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GyeryI=')))and(`qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`qr`.`__Active`=1)  limit 1000)union(select `qr`.`QuoteRequestID` from`quoterequests` `qr` join`invoices` `links_i`on`links_i`.`UserID`=`qr`.`UserID`and`links_i`.`LinkID`in('be0e4926-8e82-11e8-9cfa-12237f224cef','78891094-8e81-11e8-a116-12237f224cef','04711c56-8e81-11e8-9dcf-12237f224cef','2344153a-8e80-11e8-b396-12237f224cef','07bc92ac-8e7f-11e8-b258-12237f224cef','e52c00a6-85e6-11e8-829c-12237f224cef','1a6552e0-85dd-11e8-8349-12237f224cef','84c33270-85dc-11e8-9d64-12237f224cef','ee9876ca-85db-11e8-b36a-12237f224cef','11b53e46-85db-11e8-8221-12237f224cef','6b339b54-807a-11e8-aa89-12237f224cef','14cb5b10-789d-11e8-8e01-12237f224cef','f94b3702-789c-11e8-a1b6-12237f224cef','d65d3582-73fc-11e8-a459-12237f224cef','98d7858c-73fc-11e8-9568-12237f224cef','2f8961d6-73fc-11e8-b1e9-12237f224cef','a559ba56-73ce-11e8-adaa-12237f224cef','88dcd570-73ce-11e8-924c-12237f224cef','77cdaaf4-7011-11e8-872b-12237f224cef','524619ce-7011-11e8-b1bd-12237f224cef','1bef792e-7011-11e8-87c2-12237f224cef','2d5ab7ce-7010-11e8-874a-12237f224cef','6b8c5a44-700f-11e8-b320-12237f224cef','7e93a09c-7001-11e8-a943-12237f224cef','05b04ad6-7001-11e8-afad-12237f224cef','e8b06cfe-7000-11e8-9473-12237f224cef','cb2c2a92-7000-11e8-aa40-12237f224cef','aee830ba-7000-11e8-a2d6-12237f224cef','92d4e5d0-7000-11e8-b010-12237f224cef','64e8e522-7000-11e8-b884-12237f224cef','bf0dc99a-6ffc-11e8-98f9-12237f224cef','b0323ea2-6ffb-11e8-9413-12237f224cef','699e9864-6ffb-11e8-ab4e-12237f224cef','f4f7cb7a-6ffa-11e8-89ea-12237f224cef','e04abc8c-6ffa-11e8-8368-12237f224cef','c6c1d52a-6ffa-11e8-989a-12237f224cef','83b82aae-6ffa-11e8-b587-12237f224cef','910a4634-6ff9-11e8-a5d8-12237f224cef','90ea2f3a-6ff8-11e8-92e0-12237f224cef','c44a6b2a-6ff7-11e8-8491-12237f224cef','743b66bc-6fe7-11e8-8827-12237f224cef','5d6d6f7a-6fe7-11e8-83f4-12237f224cef','435e1058-6fe7-11e8-b16d-12237f224cef','1e7fcb48-6fe5-11e8-957b-12237f224cef','3417a228-6e4b-11e8-a132-12237f224cef')and`links_i`.`__Active`=1 left join`invoices` `_links_i`on`_links_i`.`UserID`=`qr`.`UserID`and`_links_i`.`LinkID`in('be0e4926-8e82-11e8-9cfa-12237f224cef','78891094-8e81-11e8-a116-12237f224cef','04711c56-8e81-11e8-9dcf-12237f224cef','2344153a-8e80-11e8-b396-12237f224cef','07bc92ac-8e7f-11e8-b258-12237f224cef','e52c00a6-85e6-11e8-829c-12237f224cef','1a6552e0-85dd-11e8-8349-12237f224cef','84c33270-85dc-11e8-9d64-12237f224cef','ee9876ca-85db-11e8-b36a-12237f224cef','11b53e46-85db-11e8-8221-12237f224cef','6b339b54-807a-11e8-aa89-12237f224cef','14cb5b10-789d-11e8-8e01-12237f224cef','f94b3702-789c-11e8-a1b6-12237f224cef','d65d3582-73fc-11e8-a459-12237f224cef','98d7858c-73fc-11e8-9568-12237f224cef','2f8961d6-73fc-11e8-b1e9-12237f224cef','a559ba56-73ce-11e8-adaa-12237f224cef','88dcd570-73ce-11e8-924c-12237f224cef','77cdaaf4-7011-11e8-872b-12237f224cef','524619ce-7011-11e8-b1bd-12237f224cef','1bef792e-7011-11e8-87c2-12237f224cef','2d5ab7ce-7010-11e8-874a-12237f224cef','6b8c5a44-700f-11e8-b320-12237f224cef','7e93a09c-7001-11e8-a943-12237f224cef','05b04ad6-7001-11e8-afad-12237f224cef','e8b06cfe-7000-11e8-9473-12237f224cef','cb2c2a92-7000-11e8-aa40-12237f224cef','aee830ba-7000-11e8-a2d6-12237f224cef','92d4e5d0-7000-11e8-b010-12237f224cef','64e8e522-7000-11e8-b884-12237f224cef','bf0dc99a-6ffc-11e8-98f9-12237f224cef','b0323ea2-6ffb-11e8-9413-12237f224cef','699e9864-6ffb-11e8-ab4e-12237f224cef','f4f7cb7a-6ffa-11e8-89ea-12237f224cef','e04abc8c-6ffa-11e8-8368-12237f224cef','c6c1d52a-6ffa-11e8-989a-12237f224cef','83b82aae-6ffa-11e8-b587-12237f224cef','910a4634-6ff9-11e8-a5d8-12237f224cef','90ea2f3a-6ff8-11e8-92e0-12237f224cef','c44a6b2a-6ff7-11e8-8491-12237f224cef','743b66bc-6fe7-11e8-8827-12237f224cef','5d6d6f7a-6fe7-11e8-83f4-12237f224cef','435e1058-6fe7-11e8-b16d-12237f224cef','1e7fcb48-6fe5-11e8-957b-12237f224cef','3417a228-6e4b-11e8-a132-12237f224cef')and`_links_i`.`__Active`=1 and`_links_i`.`InvoiceNumber`>`links_i`.`InvoiceNumber` left join`quoterequests` `_qr`on`_qr`.`UserID`=`qr`.`UserID`and`_qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`_qr`.`__Active`=1 and`_qr`.`QuoteRequestID`>`qr`.`QuoteRequestID`  where(`_links_i`.`InvoiceNumber`is null)and(`_qr`.`QuoteRequestID`is null)and(`qr`.`CompanyID`in(from_base64('FUGW1FiCBPI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GyeryI=')))and(`qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`qr`.`__Active`=1)  limit 1000)union(select `qr`.`QuoteRequestID` from`quoterequests` `qr` join`quoterequests` `links_qr`on`links_qr`.`UserID`=`qr`.`UserID`and`links_qr`.`LinkID`in('be0e4926-8e82-11e8-9cfa-12237f224cef','78891094-8e81-11e8-a116-12237f224cef','04711c56-8e81-11e8-9dcf-12237f224cef','2344153a-8e80-11e8-b396-12237f224cef','07bc92ac-8e7f-11e8-b258-12237f224cef','e52c00a6-85e6-11e8-829c-12237f224cef','1a6552e0-85dd-11e8-8349-12237f224cef','84c33270-85dc-11e8-9d64-12237f224cef','ee9876ca-85db-11e8-b36a-12237f224cef','11b53e46-85db-11e8-8221-12237f224cef','6b339b54-807a-11e8-aa89-12237f224cef','14cb5b10-789d-11e8-8e01-12237f224cef','f94b3702-789c-11e8-a1b6-12237f224cef','d65d3582-73fc-11e8-a459-12237f224cef','98d7858c-73fc-11e8-9568-12237f224cef','2f8961d6-73fc-11e8-b1e9-12237f224cef','a559ba56-73ce-11e8-adaa-12237f224cef','88dcd570-73ce-11e8-924c-12237f224cef','77cdaaf4-7011-11e8-872b-12237f224cef','524619ce-7011-11e8-b1bd-12237f224cef','1bef792e-7011-11e8-87c2-12237f224cef','2d5ab7ce-7010-11e8-874a-12237f224cef','6b8c5a44-700f-11e8-b320-12237f224cef','7e93a09c-7001-11e8-a943-12237f224cef','05b04ad6-7001-11e8-afad-12237f224cef','e8b06cfe-7000-11e8-9473-12237f224cef','cb2c2a92-7000-11e8-aa40-12237f224cef','aee830ba-7000-11e8-a2d6-12237f224cef','92d4e5d0-7000-11e8-b010-12237f224cef','64e8e522-7000-11e8-b884-12237f224cef','bf0dc99a-6ffc-11e8-98f9-12237f224cef','b0323ea2-6ffb-11e8-9413-12237f224cef','699e9864-6ffb-11e8-ab4e-12237f224cef','f4f7cb7a-6ffa-11e8-89ea-12237f224cef','e04abc8c-6ffa-11e8-8368-12237f224cef','c6c1d52a-6ffa-11e8-989a-12237f224cef','83b82aae-6ffa-11e8-b587-12237f224cef','910a4634-6ff9-11e8-a5d8-12237f224cef','90ea2f3a-6ff8-11e8-92e0-12237f224cef','c44a6b2a-6ff7-11e8-8491-12237f224cef','743b66bc-6fe7-11e8-8827-12237f224cef','5d6d6f7a-6fe7-11e8-83f4-12237f224cef','435e1058-6fe7-11e8-b16d-12237f224cef','1e7fcb48-6fe5-11e8-957b-12237f224cef','3417a228-6e4b-11e8-a132-12237f224cef')and`links_qr`.`__Active`=1 left join`quoterequests` `_links_qr`on`links_qr`.`UserID`=`qr`.`UserID`and`_links_qr`.`LinkID`in('be0e4926-8e82-11e8-9cfa-12237f224cef','78891094-8e81-11e8-a116-12237f224cef','04711c56-8e81-11e8-9dcf-12237f224cef','2344153a-8e80-11e8-b396-12237f224cef','07bc92ac-8e7f-11e8-b258-12237f224cef','e52c00a6-85e6-11e8-829c-12237f224cef','1a6552e0-85dd-11e8-8349-12237f224cef','84c33270-85dc-11e8-9d64-12237f224cef','ee9876ca-85db-11e8-b36a-12237f224cef','11b53e46-85db-11e8-8221-12237f224cef','6b339b54-807a-11e8-aa89-12237f224cef','14cb5b10-789d-11e8-8e01-12237f224cef','f94b3702-789c-11e8-a1b6-12237f224cef','d65d3582-73fc-11e8-a459-12237f224cef','98d7858c-73fc-11e8-9568-12237f224cef','2f8961d6-73fc-11e8-b1e9-12237f224cef','a559ba56-73ce-11e8-adaa-12237f224cef','88dcd570-73ce-11e8-924c-12237f224cef','77cdaaf4-7011-11e8-872b-12237f224cef','524619ce-7011-11e8-b1bd-12237f224cef','1bef792e-7011-11e8-87c2-12237f224cef','2d5ab7ce-7010-11e8-874a-12237f224cef','6b8c5a44-700f-11e8-b320-12237f224cef','7e93a09c-7001-11e8-a943-12237f224cef','05b04ad6-7001-11e8-afad-12237f224cef','e8b06cfe-7000-11e8-9473-12237f224cef','cb2c2a92-7000-11e8-aa40-12237f224cef','aee830ba-7000-11e8-a2d6-12237f224cef','92d4e5d0-7000-11e8-b010-12237f224cef','64e8e522-7000-11e8-b884-12237f224cef','bf0dc99a-6ffc-11e8-98f9-12237f224cef','b0323ea2-6ffb-11e8-9413-12237f224cef','699e9864-6ffb-11e8-ab4e-12237f224cef','f4f7cb7a-6ffa-11e8-89ea-12237f224cef','e04abc8c-6ffa-11e8-8368-12237f224cef','c6c1d52a-6ffa-11e8-989a-12237f224cef','83b82aae-6ffa-11e8-b587-12237f224cef','910a4634-6ff9-11e8-a5d8-12237f224cef','90ea2f3a-6ff8-11e8-92e0-12237f224cef','c44a6b2a-6ff7-11e8-8491-12237f224cef','743b66bc-6fe7-11e8-8827-12237f224cef','5d6d6f7a-6fe7-11e8-83f4-12237f224cef','435e1058-6fe7-11e8-b16d-12237f224cef','1e7fcb48-6fe5-11e8-957b-12237f224cef','3417a228-6e4b-11e8-a132-12237f224cef')and`_links_qr`.`__Active`=1 and`_links_qr`.`QuoteRequestID`>`links_qr`.`QuoteRequestID` left join`quoterequests` `_qr`on`_qr`.`UserID`=`qr`.`UserID`and`_qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`_qr`.`__Active`=1 and`_qr`.`QuoteRequestID`>`qr`.`QuoteRequestID`  where(`_links_qr`.`QuoteRequestID`is null)and(`_qr`.`QuoteRequestID`is null)and(`qr`.`CompanyID`in(from_base64('FUGW1FiCBPI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GyeryI=')))and(`qr`.`CompanyID`in(from_base64('FUGW1CgJrPI='),from_base64('FUGW1CwXsGI='),from_base64('FUGW1DAdiBI='),from_base64('FUGW1ALlC0I='),from_base64('FUGW1Afct/I='),from_base64('FUGW1AvY4hI='),from_base64('FUGW1A/VHWI='),from_base64('FUGW1ITiiyI='),from_base64('FUGW1HCqCEI='),from_base64('FUGW1HS1YQI='),from_base64('FUGW1HjE/MI='),from_base64('FUGW1HzY03I='),from_base64('FUGW1FiCBPI='),from_base64('FUGW1FyG6nI='),from_base64('FUGW1GCK+OI='),from_base64('FUGW1GSPrGI='),from_base64('FUGW1GyeryI='),from_base64('FUGW1ERJC7I='),from_base64('FUGW1EhjGAI='),from_base64('FUGW1ExnO0I='),from_base64('FUGW1FBv/FI='),from_base64('FUGW1BPaCtI='),from_base64('FUGW1BfkosI='),from_base64('FUGW1BvvLaI='),from_base64('FUGW1CAAssI='),from_base64('FUGW1CQIILI='),from_base64('FUGW1Dg111I='),from_base64('FUGW1DxB1kI='),from_base64('FUGW1EBBlMI='),from_base64('FUGW1FR2UVI='),from_base64('FUGW1DQpvnI='),from_base64('FUGW1IDiq/I='),from_base64('FUGW1GiYoLI='))and`qr`.`__Active`=1)  limit 1000))a join`quoterequests` `qr`using(`QuoteRequestID`)   limit 1000)a;";

fn mysql_format(mysql: &str) -> String {
    let bytes = mysql.as_bytes();
    let len = mysql.len();
    let mut s = String::with_capacity(len * 8);
    s.push_str("<pre>");
    let mut cur_token = Token::Unset;
    let mut prev_token = Token::Unset;
    let mut token_start = 0usize;
    let mut token_end = 0usize;
    let mut escaped = false;
    let mut quote = '\'';
    let mut p = 0;
    let mut prev_str = String::new();
    let mut word = String::new();
    let mut skip = 0u8;
    let mut end_tag: &str = "";
    let mut token_ready = false;

    for (i, b) in bytes.iter().enumerate() {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        macro_rules! push_newline {
            () => {
                s.push_str(&format!("\n{}", " ".repeat(2 * p)));
            };
        }

        // push a token to the formatted output
        macro_rules! push_token {
            () => {
                if prev_token != Token::Unset && prev_token != cur_token {
                    s.push_str(end_tag)
                }
                match cur_token {
                    Token::Unset => {
                        break;
                    }
                    Token::Word => {
                        match prev_token {
                            Token::Word
                            | Token::Number
                            | Token::Hex
                            | Token::HexString
                            | Token::System
                            | Token::Variable => s.push(' '),
                            _ => {}
                        }
                        match word.as_str() {
                            "from" | "where" | "and" | "or" | "order" | "group" | "having"
                            | "limit" | "straight_join" | "cross" | "natural" | "union" => {
                                push_newline!()
                            }
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
                        if prev_token != cur_token {
                            s.push_str("<b style=\"color:#1976d2\">");
                        }
                        s.push_str(&word);
                        end_tag = "</b>";
                        prev_str = word.clone();
                    }
                    Token::Function => {
                        match prev_token {
                            Token::Word
                            | Token::Number
                            | Token::Hex
                            | Token::HexString
                            | Token::System
                            | Token::Variable => s.push(' '),
                            _ => {}
                        }
                        if prev_token != cur_token {
                            s.push_str("<span style=\"color:#d81b60\">");
                        }
                        s.push_str(&word);
                        end_tag = "</span>";
                    }
                    Token::String => {
                        if prev_token != cur_token {
                            s.push_str("<span style=\"color:#2e7d32\">'");
                        }
                        if quote == '\'' {
                            s.push_str(&encode_minimal(&mysql[token_start..=token_end]));
                        } else {
                            let q = &quote.to_string();
                            s.push_str(&encode_minimal(
                                &mysql[token_start..=token_end]
                                    .replace(&format!("{}{}", "\\", q).to_owned(), q)
                                    .replace("'", "\\'"),
                            ));
                        }
                        end_tag = "'</span>";
                    }
                    Token::Name => {
                        match prev_token {
                            Token::Name => s.push(' '),
                            _ => {}
                        }
                        if prev_token != cur_token {
                            s.push_str("<span style=\"color:#795548\">");
                        }
                        s.push('`');
                        s.push_str(&encode_minimal(&mysql[token_start..=token_end]));
                        s.push('`');
                        end_tag = "</span>";
                    }
                    Token::Number => {
                        match prev_token {
                            Token::Word
                            | Token::Hex
                            | Token::HexString
                            | Token::System
                            | Token::Variable => s.push(' '),
                            _ => {}
                        }
                        if prev_token != cur_token {
                            s.push_str("<span style=\"color:#ce4800\">");
                        }
                        s.push_str(&mysql[token_start..=token_end]);
                        end_tag = "</span>";
                    }
                    Token::Hex | Token::HexString => {
                        match prev_token {
                            Token::Word => s.push(' '),
                            _ => {}
                        }
                        if prev_token != cur_token {
                            s.push_str(
                                "<span style=\"color:#673ab7;background-color:#f0f0f0\">",
                            );
                        }
                        s.push_str(&mysql[token_start..=token_end]);
                        end_tag = "</span>";
                    }
                    Token::Symbol => {
                        if prev_token != cur_token {
                            s.push_str("<b>");
                        }
                        s.push_str(&encode_minimal(&format!("{}", bytes[i] as char)));
                        end_tag = "</b>";

                        // prev_char = bytes[i] as char;
                    }
                    Token::System => {
                        match prev_token {
                            Token::Word
                            | Token::Hex
                            | Token::HexString
                            | Token::System
                            | Token::Variable
                            | Token::Number => s.push(' '),
                            _ => {}
                        }
                        if prev_token != cur_token {
                            s.push_str("<i style=\"color:#00838f\">");
                        }
                        s.push_str(&mysql[token_start..=token_end]);
                        end_tag = "</i>";
                    }
                    Token::Variable => {
                        match prev_token {
                            Token::Word
                            | Token::Hex
                            | Token::HexString
                            | Token::System
                            | Token::Variable
                            | Token::Number => s.push(' '),
                            _ => {}
                        }
                        if prev_token != cur_token {
                            s.push_str("<span style=\"color:#757575\">");
                        }
                        s.push_str(&mysql[token_start..=token_end]);
                        end_tag = "</span>";
                    }
                }
                prev_token = cur_token;
                cur_token = Token::Unset;
                token_ready = false;
            };
        }

        // make sure current byte is valid for cur_token
        match cur_token {
            Token::Unset => {}
            Token::Word | Token::Function => match *b as char {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {}
                _ => {
                    token_end = i - 1;
                    word = mysql[token_start..=token_end].to_ascii_lowercase();
                    if *b as char == '(' {
                        if is_function(&word) {
                            cur_token = Token::Function;
                        } else if !is_word(&word) {
                            cur_token = Token::Name;
                        }
                    } else if !is_word(&word) {
                        cur_token = Token::Name;
                    }
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
            Token::System | Token::Variable => match *b as char {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '_' => {}
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

                                    token_ready = true;
                                }
                            }
                        }
                        _ => {}
                    }

                    if !token_ready {
                        cur_token = Token::Word;
                        token_start = i;
                    }
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

                                    token_ready = true;
                                }
                            }
                        }
                        _ => {}
                    }

                    if !token_ready {
                        cur_token = Token::Number;
                        token_start = i;
                    }
                }
                '.' | '=' | ';' | '(' | ')' | '?' | '^' | '&' | '|' | '/' | '*' | '+' | '-'
                | ':' | '~' | '<' | '>' | '!' | '%' | ',' => {
                    match *b as char {
                        '(' => p += 1,
                        ')' => p -= 1,
                        '+' | '-' | '.' => {
                            if i + 1 < len {
                                match bytes[i + 1] as char {
                                    '0'...'9' | '.' => {
                                        cur_token = Token::Number;
                                        token_start = i;

                                        token_ready = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }

                    if !token_ready {
                        cur_token = Token::Symbol;
                        push_token!();
                    }
                }
                c if c.is_whitespace() => {}
                '@' => {
                    token_start = i;
                    if i + 1 < len && bytes[i + 1] as char == '@' {
                        cur_token = Token::System;
                        skip = 1;
                    } else {
                        cur_token = Token::Variable;
                    }
                }
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
    s.push_str("</pre>");

    return s;
}

fn main() {
    let start = PreciseTime::now();
    let s = mysql_format(_MYSQL3);
    let end = PreciseTime::now();

    println!("{}", s);
    println!("Took {}", start.to(end));

    fs::write("debug.html", s).expect("Unable to write file");
}
