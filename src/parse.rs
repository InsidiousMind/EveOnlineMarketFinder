use std::path::Path;
use csv;
use rustc_serialize;

#[derive(Debug)]
pub struct Items {
    pub ids: Vec<Item>,
    csv_path: String,
}

#[derive(RustcDecodable, Debug)]
pub struct Item {
    pub typeID: i64,
    pub groupID: i64,
    pub typeName: String,
    pub description: Option<String>,
    pub mass: f64,
    pub volume: Option<i64>,
    pub capacity: Option<i64>,
    pub portionSize: Option<i64>,
    pub raceID: Option<i64>,
    pub basePrice: Option<i64>,
    pub published: Option<i64>,
    pub marketGroupID: Option<i64>,
    pub iconID: Option<i64>,
    pub soundID: Option<i64>,
    pub graphicID: Option<i64>,
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

