extern crate serialize;
use serialize::json;
use std::str;

#[deriving(Decodable, Encodable)]
pub struct FauxPerson {
    name: String,
    surname: String,
    gender: String,
    country: String,
}

fn main() {
    let mut sock_names = std::io::TcpStream::connect("api.uinames.com:80").unwrap();
    let http_req_status = sock_names.write(b"GET http://api.uinames.com/?amount=6 HTTP/1.0\n\n");

    match http_req_status {
        Ok(x) => println!(""),
        Err(x) => panic!(x),
    }

    let resp_names_result = sock_names.read_to_end();

    let resp_names = match resp_names_result {
        Ok(t) => t,
        Err(e) => panic!(e),
    };

    let names_structure = match str::from_utf8(resp_names.as_slice()) {
        Some(e) => e,
        None => panic!("Invalid UTF-8 sequence"),
    };

    let http_pieces = names_structure.split_str("\n");
    let parts: Vec<&str> = http_pieces.collect();
    let names_body = parts[parts.len() - 1];

    let people: Vec<FauxPerson> = json::decode(names_body.as_slice()).unwrap();
    for who in people.iter() {
        println!("{} {}\n{} from {}\n", who.name, who.surname, who.gender, who.country);
    }
}

