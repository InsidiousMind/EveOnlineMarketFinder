extern crate requests;
use requests::ToJson;

fn main() {
    let response = requests::get("http://api.eve-central.com/api/marketstat?typeid=34&typeid=35&regionlimit=10000002").unwrap();
    assert_eq!(response.url(), "http://api.eve-central.com/api/marketstat?typeid=34&typeid=35&regionlimit=10000002");
    assert_eq!(response.reason(), "OK");
    assert_eq!(response.status_code(), requests::StatusCode::Ok);

    //let data = response.json().unwrap();
    println!("response: {:?}", response);
    //println!("data: {}", data);
       

}
