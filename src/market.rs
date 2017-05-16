use ansi_term::Colour::{Red, Yellow, Blue};
use hyper::{Client, Url};
use hyper::client::Response;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use std::io::Read;

use serde_json;
use hyper;

pub struct MarketItems {
    pub market_items: Vec<MarketItem>,
    pub item_info: Option<Vec<ItemsWrapper>>,
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
    #[serde(rename(deserialize="totalCount_str"))]
    pub total_count_str: String,
    #[serde(rename(deserialize="pageCount"))]
    pub page_count: i64,
    pub items: Vec<Items>,
    pub next: Option<Navigation>,
    pub previous: Option<Navigation>,
    #[serde(rename(deserialize="totalCount"))]
    pub total_count: i64,
    #[serde(rename(deserialize="pageCount_str"))]
    pub page_count_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Navigation {
    href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    #[serde(rename(deserialize="marketGroup"))]
    market_group: MarketGroup,
    #[serde(rename(deserialize="type"))]
    item_type: Types,
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
            item_info: None, // Get rid of option, b/c vector always vec
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
                // TODO optimize this
                let mut total_item_count = 0;
                for page in types.iter() {
                    for item in page.items.iter() {
                        self.market_items.push(
                            MarketItem { 
                                type_id: item.item_type.id, 
                                history: serde_json::from_str(&Self::request_data(&self.client, item.item_type.id, total_item_count)).unwrap()
                            });

                            println!("{} Info for Type: '{}' Downloaded ID: {}", 
                                    Blue.bold().paint("[Query] --"), 
                                    Yellow.paint(item.item_type.name.clone()), 
                                    item.item_type.id);
                            total_item_count += 1;
                    }
                } //inner
            },
        }
    }


    fn request_data(client: &Client, typeid: i64, item_count: i64) -> String {
        
        let root_url = "https://esi.tech.ccp.is/latest/markets/10000002/history/?datasource=tranquility";
        //the endpoint on ESI we will be querying
        let endpoint = Url::parse_with_params(&root_url,
                                                  &[("type_id", typeid.to_string())]).unwrap();
        // send the GET request
        let mut res = client.get(endpoint.as_ref()).send().unwrap();
        assert_eq!(res.status, hyper::Ok, 
                   "\n {} The API Request Did Not Work status: {} at endpoint: {}. Total Item Count: {}", 
                   Red.bold().paint("[ERROR]"), 
                   Red.paint(res.status.to_string()),
                   Red.bold().paint(endpoint.as_ref().to_string()),
                   Blue.bold().paint(item_count.to_string()),
                   );
        
        //convert result to String
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    fn get_items(&mut self) {

        let mut body = String::new();
        let mut res: Option<Response> = None;
        let root_url: &str = "https://crest-tq.eveonline.com/market/types/";
        match self.item_info {
            None => {
                println!("{}", Blue.bold().paint("Downloading Item Types From First Page..."));
                let endpoint = Url::parse(root_url).unwrap();
                res = Some(self.client.get(endpoint.as_ref()).send().unwrap());
                assert_eq!(res.as_ref().unwrap().status, hyper::Ok,
                           "\n {} The API Request Did Not Work, status: {}, for endpoint {}",
                           Red.bold().paint("[ERROR]"),
                           res.as_ref().unwrap().status,
                           &endpoint.to_string());
            } 
            Some(ref mut item_vec) => {
                for x in 2...item_vec.get(0).unwrap().page_count {
                    let endpoint: Url = Url::parse_with_params(root_url, &[("page", x.to_string())]).unwrap();  
                    println!("{} Downloading Item Info from Page: {}", Blue.paint("[QUERY] --"), Yellow.bold().paint(endpoint.as_ref().to_string()));

                    res = Some(self.client.get(endpoint.as_ref()).send().unwrap());
                    assert_eq!(res.as_ref().unwrap().status, hyper::Ok,
                               "\n {} The API Request Did Not Work, status: {} for endpoint {}",
                               Red.bold().paint("[ERROR]"),
                               res.as_ref().unwrap().status,
                               &endpoint.to_string());
                }
            }
        }

        res.unwrap().read_to_string(&mut body).unwrap();
        match self.item_info {
            None => {
                self.item_info = Some(Vec::new()); 
                self.item_info.as_mut().unwrap().push(serde_json::from_str(&body).unwrap());
                self.get_items();
            },

            Some(ref mut item_vec) => {
                let json = serde_json::from_str(&body).unwrap();
                item_vec.push(json);  
            }

         
        }
        //self.item_info.as_mut().unwrap().push(serde_json::from_str(&body).unwrap());
    }
}

