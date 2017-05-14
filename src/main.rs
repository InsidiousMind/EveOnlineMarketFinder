#![feature(rustc_private)]
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate rustc_serialize;

#[macro_use]
extern crate serde_derive;

use hyper::{ Client, Url} ;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

use parse::Items;

mod parse;

struct MarketItems {
    type_id: i64,
    history: Vec<MarketItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketItem {
    date: String,
    order_count: i64,
    volume: i64,
    highest: f64,
    average: f64,
    lowest: f64,
}
/*
enum Hubs {
    TheForge:   { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Jita", },
    Domain:     { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Amarr"},
    Heimatar:   { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Rens"},
    SinqLaison: { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Dodixie"},
    Metropolis: { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Hek"},
    Essence:    { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Oursulaert"},
    TashMurkon: { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Tash-Murkon Prime"},
    Khanid:     { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Agil"},
}
*/

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let items = Items::new(String::from("./data/invTypes.csv")).parse_csv();
    let data = Vec::<MarketItems>::new();
    
    items.ids.iter().map( |x| {
        let temp = MarketItems { type_id: x.typeID, history: serde_json::from_str(&request_data(client, x.typeID)).unwrap()};
        data.push(temp);
    }).collect();
}

fn request_data(client: Client, typeid: i64) -> String {

    let mut endpoint = Url::parse_with_params("https://esi.tech.ccp.is/latest/markets/10000002/history/?datasource=tranquility",
                                              &[("type_id", typeid.to_string())]).unwrap();

    let mut res = client.get(endpoint).send().unwrap();
    assert_eq!(res.status, hyper::Ok, "[ERROR] The API Request Did Not Work {}", res.status);

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}
