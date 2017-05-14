#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;

extern crate hyper;

use serde_xml_rs::deserialize;
use hyper::Client;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct EvecApi {
    version: String,
    method: String,
    marketstat: MarketStat,
}

#[derive(Debug, Deserialize)]
struct MarketStat {
    #[serde(rename="type")]
    types: Vec<mType>,
}

#[derive(Debug, Deserialize)]
struct mType {
    id: i64,
    buy: Data,
    sell: Data,
    all: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    volume: f64,
    avg: f64,
    max: f64,
    min: f64,
    stddev: f64,
    median: f64,
    percentile: f64,
}

enum Hubs {
    TheForge:   { id: i64, system: "Jita",              hub_id: i64},
    Domain:     { id: i64, system: "Amarr",             hub_id: i64},
    Heimatar:   { id: i64, system: "Rens",              hub_id: i64},
    SinqLaison: { id: i64, system: "Dodixie",           hub_id: i64},
    Metropolis: { id: i64, system: "Hek",               hub_id: i64},
    Essence:    { id: i64, system: "Oursulaert",        hub_id: i64},
    TashMurkon: { id: i64, system: "Tash-Murkon Prime", hub_id: i64},
    Khanid:     { id: i64, system: "Agil",              hub_id: i64},
}

fn main() {
    let url = "http://api.eve-central.com/api/marketstat?typeid=34&typeid=35&regionlimit=10000002";
    let client = Client::new();

    let mut res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok, "[ERROR NOT OK] The API seems to be down");
    
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let eve_api: EvecApi = deserialize(body.as_bytes()).unwrap();
    println!("{:#?}", eve_api);
}

fn request_data(reg_id: i64, sys_id: i64, t_ids: Vec<i64>, ) {
    let client = Client::new(); 

    let mut res = client.get(form_url(reg_id, sys_id, t_ids)).send().unwrap();
    assert_eq!(res.status, hyper::ok, "[ERROR] The API Request Did Not Work");
    
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let eve_api: EvecApi = deserialize(body.as_bytes()).unwrap();

}

fn form_url(reg_id: i64, sys_id: i64, t_ids: Vec<i64>) -> String {
    unimplemented!();
}


