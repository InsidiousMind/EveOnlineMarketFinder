use ansi_term::Colour::{Red, Yellow, Blue, Purple};
use hyper::{Client, Url};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use std::io::Read;

use serde_json;
use hyper;

pub struct MarketItems {
    pub market_items: Vec<MarketItem>,
    pub item_info: Option<ItemsWrapper>,
    pub client: Client,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsWrapper {
    pub totalCount_str: String,
    pub pageCount: i64,
    pub items: Vec<Items>,
    pub next: Next,
    pub totalCount: i64,
    pub pageCount_str: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Next {
    href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    marketGroup: MarketGroup,
    itemType: Types,
    id: i64,
    id_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketGroup {
    href: Option<String>,
    id: i64,
    id_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Types {
    id_str: String,
    href: Option<String>,
    id: i64,
    name: String,
    icon: Icon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    href: Option<String>,
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

    pub fn new() -> Self {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        MarketItems{
            market_items: Vec::new(),
            item_info: None,
            client: client,
        }
    }

    pub fn query_items(&mut self) {
        
        //iterate through ID's and query it, store result in data vec
        let mut count = 0; 
        match self.item_info {
            None => {
                println!("{} Item Info Does Not Exist", Yellow.bold().paint("[WARNING]"));
                self.get_items();
                count += 1;
                self.query_items();
                if count == 5 { panic!("{} Could Not Succesfully Complete Item Query", Red.bold().paint("[ERROR]")); }
            },
            Some(ref mut types) => {
                for item in types.items.iter() {
                    self.market_items.push(
                        MarketItem { 
                            type_id: item.itemType.id, 
                            history: serde_json::from_str(&Self::request_data(&self.client, item.itemType.id)).unwrap()
                        });
            
                        println!("{} Info for Type: '{}' Downloaded ID: {}", 
                                Blue.bold().paint("[Query] --"), 
                                Yellow.paint(item.itemType.name.clone()), 
                                item.itemType.id);
                }
            },
        }
    }


    fn request_data(client: &Client, typeid: i64) -> String {
        
        let root_url = "https://esi.tech.ccp.is/latest/markets/10000002/history/?datasource=tranquility";
        //the endpoint on ESI we will be querying
        let endpoint = Url::parse_with_params(&root_url,
                                                  &[("type_id", typeid.to_string())]).unwrap();
        // send the GET request
        let mut res = client.get(endpoint).send().unwrap();
        assert_eq!(res.status, hyper::Ok, 
                   "\n {} The API Request Did Not Work status: {} at endpoint: {}", 
                   Red.bold().paint("[ERROR]"), 
                   res.status,
                   Red.bold().paint(root_url),
                   );
        
        //convert result to String
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    fn get_items(&mut self) {

        println!("{}", Blue.bold().paint("Downloading Item Types..."));
        
        let endpoint = Url::parse("https://crest-tq.eveonline.com/market/types/").unwrap();
        let mut res = self.client.get(endpoint).send().unwrap();

        assert_eq!(res.status, hyper::Ok,
                   "\n {} The API Request Did Not Work, status: {}",
                   Red.bold().paint("[ERROR]"),
                   res.status);
    
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        self.item_info = serde_json::from_str(&body).unwrap();
    }
}

