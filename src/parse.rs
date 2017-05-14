use std::path::Path;
use csv;
use rustc_serialize;

pub struct Items {
    pub ids: Vec<Item>,
    csv_path: String,
}

#[derive(RustcDecodable, Debug)]
struct Item {
    typeID: i64,
    groupID: i64,
    typeName: String,
    description: Option<String>,
    mass: f64,
    volume: Option<i64>,
    capacity: Option<i64>,
    portionSize: Option<i64>,
    raceID: Option<i64>,
    basePrice: Option<i64>,
    published: Option<i64>,
    marketGroupID: Option<i64>,
    iconID: Option<i64>,
    soundID: Option<i64>,
    graphicID: Option<i64>,
}

impl Items {

    pub fn new(csv_path: String) -> Self {

        Items {
            ids: Vec::new(),
            csv_path: csv_path,
        }
    }

    pub fn parse_csv(&mut self) {

        println!("PATH: {}", self.csv_path);
        let mut rdr = csv::Reader::from_file(&self.csv_path).unwrap().has_headers(true);
        self.ids = rdr.decode().collect::<csv::Result<Vec<Item>>>().unwrap();
    }

    fn print(&self) {
        for item in self.ids.iter() {
            println!("{:?}", item); 
        } 
    }
}

