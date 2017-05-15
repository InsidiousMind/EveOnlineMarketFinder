#![feature(rustc_private)]
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate rustc_serialize;
extern crate ansi_term;

#[macro_use]
extern crate serde_derive;

use hyper::{ Client, Url} ;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use parse::Items;
use market::MarketItems;

mod parse;
mod market;

fn main() {
    //set up the client for SSL connection
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    // parse the CSV data
    let mut items = Items::new(String::from("./data/invTypes.csv"));
    items.parse_csv();

    let mut market = MarketItems::new(items);
    market.query_items(client);
}

