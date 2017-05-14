extern crate hyper;
use hyper::Client;

fn main() {
    let url = "http://api.eve-central.com/api/marketstat?typeid=34&typeid=35&regionlimit=10000002";
    let client = Client::new();

    let res = client.get(url).send().unwrap();
    assert_eq!(res.status, hyper::Ok);
}
