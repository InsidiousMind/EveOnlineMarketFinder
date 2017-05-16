extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate rustc_serialize;
extern crate ansi_term;

#[macro_use]
extern crate serde_derive;

use market::MarketItems;

mod market;

fn main() {
    //set up the client for SSL connection
    let mut market = MarketItems::new();
    market.query_items();

    /* parse the CSV data
    let mut items = Items::new(String::from("./data/invTypes.csv"));
    items.parse_csv();
    */

}

