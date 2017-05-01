extern crate hyper;
extern crate hyper_native_tls;
extern crate cgi;
extern crate scraper;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;
use scraper::{Html, Selector};

fn main() {
    println!("content-type: text/html\n");

    if cgi::get_header("REQUEST_METHOD") == "GET"{
        println!("<form method=\"POST\">Url:<input name=\"url\" type=\"text\"></input><br/>Selector:<input name=\"sel\" type=\"text\"></input><button>submit</button></form>")
    } else {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        let mut content = String::new();
        let form = cgi::get_payload_form_data();
        let url:&str = form.get("url").unwrap();
	let selector = Selector::parse(form.get("sel").unwrap()).unwrap();
        println!("<title>PageQuery: {}</title>", url);
        client.get(url).send().unwrap().read_to_string(&mut content).unwrap();
	let document = Html::parse_document(&content[..]);
	for element in document.select(&selector){
            println!("{}<br/><br/>", element.html());
        }
        //println!("{}", content)
    }
}
