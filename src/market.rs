use parse::Items;
use ansi_term::Colour::{Red, Yellow, Blue};
use hyper::{Client, Url};
use std::io::Read;

use serde_json;
use hyper;

pub struct MarketItems {
    pub market_items: Vec<MarketItem>,
    pub item_info: Items,
}

#[derive(Debug)]
pub struct MarketItem {
    pub type_id: i64,
    pub history: Vec<ItemData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemData {
    pub date: String,
    pub order_count: i64,
    pub volume: i64,
    pub highest: f64,
    pub average: f64,
    pub lowest: f64,
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

impl MarketItems {

    pub fn new(items: Items) -> Self {

        MarketItems{
            market_items: Vec::new(),
            item_info: items,
        }
    }

    fn request_data(client: &Client, typeid: i64) -> String {
        
        //the endpoint on ESI we will be querying
        let endpoint = Url::parse_with_params("https://esi.tech.ccp.is/latest/markets/10000002/history/?datasource=tranquility",
                                                  &[("type_id", typeid.to_string())]).unwrap();
        // send the GET request
        let mut res = client.get(endpoint).send().unwrap();
        
        assert_eq!(res.status, hyper::Ok, 
                   "\n {} The API Request Did Not Work status: {}", 
                   Red.bold().paint("[ERROR]"), 
                   res.status);
        
        //convert result to String
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    pub fn query_items(&mut self, client: Client) {
        
        //iterate thorugh ID's and query it, store result in data vec
        for id in self.item_info.ids.iter() {
            self.market_items.push(MarketItem { type_id: id.typeID, history: serde_json::from_str(&Self::request_data(&client, id.typeID)).unwrap()});
            
            println!("{} Info for Type: '{}' Downloaded ID: {}", 
                     Blue.bold().paint("[Query] --"), 
                     Yellow.paint(id.typeName.clone()), 
                     id.typeID);
        }
    }
}

