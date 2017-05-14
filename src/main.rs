#[macro_use] extern crate serde_derive;
extern crate serde_xml_rs;

extern crate hyper;

use serde_xml_rs::deserialize;
use hyper::Client;
use hyper::Url;
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
    TheForge:   { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Jita", },
    Domain:     { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Amarr"},
    Heimatar:   { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Rens"},
    SinqLaison: { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Dodixie"},
    Metropolis: { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Hek"},
    Essence:    { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Oursulaert"},
    TashMurkon: { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Tash-Murkon Prime"},
    Khanid:     { RegionLimit("regionlimit", i64), System("usesystem", i64), SystemName: "Agil"},
}

fn main() {
    let url = "http://api.eve-central.com/api/marketstat?typeid=34&typeid=35&regionlimit=10000002";


}

fn request_data(reg_id: i64, sys_id: i64, t_ids: Vec<i64>, ) {
    let client = Client::new(); 
    
    let mut endpoint = Url::parse_with_params("http://api.eve-central.com/api/marketstat",
                                              &[]
                                              );
    match endpoing {
        Some() 
    }
    

    let mut res = client.get(form_url(reg_id, sys_id, t_ids)).send().unwrap();
    
    assert_eq!(res.status, hyper::ok, "[ERROR] The API Request Did Not Work");
    
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let eve_api: EvecApi = deserialize(body.as_bytes()).unwrap();

}

fn form_url(reg_id: i64, sys_id: i64, t_ids: Vec<i64>) -> String {
    let base_url = "http://api.eve-central.com/api/marketstat?typeid=34&typeid=35&regionlimit=";
}


