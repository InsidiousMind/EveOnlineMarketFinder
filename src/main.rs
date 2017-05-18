#![feature(inclusive_range_syntax)]
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate rustc_serialize;
extern crate ansi_term;

#[macro_use]
extern crate serde_derive;

use market::Market;

mod market;

fn main() {
    let mut market = Market::new("Jita").unwrap();
    market.get_items();

//    println!("Types! {:?}", market.market_items.get(0).unwrap().type_history.get(&2817));

}

