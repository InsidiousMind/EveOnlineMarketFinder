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

#[derive(Debug)]
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
    //set up the client for SSL connection
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    // parse the CSV data
    let mut items = Items::new(String::from("./data/invTypes.csv"));
    items.parse_csv();
    
    //setup the vector which will hold all the market data
    let mut data = Vec::<MarketItems>::new();

    //iterate thorugh ID's and query it, store result in data vec
    for id in items.ids.iter() {
        data.push(MarketItems { type_id: id.typeID, history: serde_json::from_str(&request_data(&client, id.typeID)).unwrap()});
        println!("TypeID: {}", id.typeID);
    }
}

fn request_data(client: &Client, typeid: i64) -> String {
    
    //the endpoint on ESI we will be querying
    let mut endpoint = Url::parse_with_params("https://esi.tech.ccp.is/latest/markets/10000002/history/?datasource=tranquility",
                                              &[("type_id", typeid.to_string())]).unwrap();
    // send the GET request
    let mut res = client.get(endpoint).send().unwrap();
    assert_eq!(res.status, hyper::Ok, "[ERROR] The API Request Did Not Work {}", res.status);
    
    //convert result to String
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}
