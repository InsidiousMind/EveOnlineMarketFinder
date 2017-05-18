use ansi_term::Colour::{Red, Yellow, Blue};
use hyper::{Client, Url};
use hyper::client::Response;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use std::io::Read;
use std::collections::HashMap;

use serde_json;
use hyper;

pub struct Market {
    pub market_items: Vec<Aggregates>,
    pub item_info: Option<Vec<ItemsWrapper>>,
    pub client: Client,
    pub station_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aggregates {
    pub type_history: HashMap<i64, MarketData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub buy: Data,
    pub sell: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename(deserialize="weightedAverage"))]
    weighted_average: Option<String>,
    max: Option<String>,
    min: Option<String>,
    #[serde(rename(deserialize="stddev"))]
    std_dev: Option<String>,
    median: Option<String>,
    volume: Option<String>,
    #[serde(rename(deserialize="orderCount"))]
    order_count: Option<String>,
    percentile: Option<String>,
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

#[derive(Copy, Clone)]
pub enum Hubs {
    Jita,
    Amarr,
    Dodixie,
    Rens,
    Hek,
}

impl Hubs {

    // uppercase version of name for use in `match` statements
    fn name_match(&self) -> String {
        match *self {
            Hubs::Jita    => "Jita".to_uppercase(),
            Hubs::Amarr   => "Amarr".to_uppercase(),
            Hubs::Dodixie => "Dodixie".to_uppercase(),
            Hubs::Rens    => "Rens".to_uppercase(),
            Hubs::Hek     => "Hek".to_uppercase(),
        }
    }

    fn id(&self) -> i64 {
        match *self {
            Hubs::Jita    => 60003760,
            Hubs::Amarr   => 60008494,
            Hubs::Dodixie => 60011866,
            Hubs::Rens    => 60004588,
            Hubs::Hek     => 60005686,
        }
    }

    fn id_str(&self) -> &'static str {
        match *self {
            Hubs::Jita    => "60003760",
            Hubs::Amarr   => "60008494",
            Hubs::Dodixie => "60011866",
            Hubs::Rens    => "60004588",
            Hubs::Hek     => "60005686",
        }
    }

    fn name(&self) -> &str {
        match *self {
            Hubs::Jita    => "Jita",
            Hubs::Amarr   => "Amarr",
            Hubs::Dodixie => "Dodixie",
            Hubs::Rens    => "Rens",
            Hubs::Hek     => "Hek",
        }
    }
}

//static, pub, private
impl Market {

    pub fn new(station: &str) -> Option<Self> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
       
        match Self::parse_hubs(station) {
            Some(x) => {
                Some(Market {
                    market_items: Vec::new(),
                    item_info: None,
                    client: client,
                    station_id: x.to_owned(),
                })
            }
            None => { 
                println!("{} Please Enter The Station Name Correctly. Proceed with Default (Jita) Station? (Y/n)", Red.bold().paint("[ERROR]")); 
                return None;
            }
        }
    }

    fn parse_hubs(station: &str) -> Option<&str> {
        match String::from(station).to_uppercase().as_ref() {
            station_id if station_id == Hubs::Jita.name_match()    => Some(Hubs::Jita.id_str().clone()),
            station_id if station_id == Hubs::Amarr.name_match()   => Some(Hubs::Amarr.id_str().clone()),
            station_id if station_id == Hubs::Dodixie.name_match() => Some(Hubs::Dodixie.id_str().clone()),
            station_id if station_id == Hubs::Rens.name_match()    => Some(Hubs::Dodixie.id_str().clone()),
            station_id if station_id == Hubs::Hek.name_match()     => Some(Hubs::Hek.id_str().clone()),
            _ =>  None,
        }
    }

    fn request_data(items: &Vec<Items>, station_id: &str, client: &Client, mut item_count: i64) -> String {
        
        //the endpoint on Fuzzwork we will be querying 
        let root_url = "https://market.fuzzwork.co.uk/aggregates/?region=".to_string() + station_id;
        let endpoint = Url::parse_with_params(&root_url, &[("types", Self::form_item_url(items, &mut item_count))]).unwrap();
        
        // send the GET request
        let mut res = client.get(endpoint.as_ref()).send().unwrap();
        assert_eq!(res.status, hyper::Ok, 
                   "\n {} The API Request Did Not Work status: {} at endpoint: {}. \n Total Item Count: {}", 
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
    
    fn form_item_url(items: &Vec<Items>, item_count: &mut i64) -> String {
        let mut items_str = "".to_string();
        for item in items.iter() {
            //a trailing comma will not produce HTTP error
            let temp = (&item.item_type.id_str).to_string() + ",";
            items_str += &temp;
            *item_count += 1;
        }
        return items_str;
    }
    
    pub fn get_items(&mut self) {
        
        //iterate through ID's and query it, store result in data vec
        let mut count = 0; 
        match self.item_info {
            None => {
                println!("{} Item Info Does Not Exist", Yellow.bold().paint("[WARNING]"));
                self.request_items();
                count += 1;
                self.get_items();
                if count == 5 { panic!("{} Could Not Succesfully Complete Item Query", Red.bold().paint("[ERROR]")); }
            },
            Some(ref mut types) => {
                // TODO optimize this
                let total_item_count = 0;
                println!("{} Downloading information for all types", Blue.bold().paint("[Query] --"));
                for page in types.iter() {
                    self.market_items.push(
                        Aggregates { 
                            type_history: serde_json::from_str(&Self::request_data(page.items.as_ref(), self.station_id.as_ref(), &self.client, total_item_count).as_str()).unwrap()
                        });
                }
            },
        }
    }

    fn request_items(&mut self) {
        let mut body = String::new();
        let mut res: Option<Response> = None;
        let root_url: &str = "https://crest-tq.eveonline.com/market/types/";
        match self.item_info {
            None => {
                println!("{}", Blue.bold().paint("Downloading item types (this may take a few seconds...)"));
                let endpoint = Url::parse(root_url).unwrap();
                res = Some(self.client.get(endpoint.as_ref()).send().unwrap());
                assert_eq!(res.as_ref().unwrap().status, hyper::Ok,
                           "\n {} The API request didn't work, status: {}, for endpoint {}",
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
                               "\n {} The API request didn't work, status: {} for endpoint {}",
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
                self.request_items();
            },
            Some(ref mut item_vec) => {
                let json = serde_json::from_str(&body).unwrap();
                item_vec.push(json);  
            }
        }
    }

    fn find(&self) -> Vec<Items>{
        unimplemented!(); 
    
    }
}

