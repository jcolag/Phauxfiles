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
    let names = http_get("api.uinames.com:80", b"GET http://api.uinames.com/?amount=6 HTTP/1.0\n\n");
    let people: Vec<FauxPerson> = json::decode(names.as_slice()).unwrap();
    for who in people.iter() {
        println!("{} {}\n{} from {}\n", who.name, who.surname, who.gender, who.country);
    }
}

fn http_get(server: &str, request: &[u8]) -> String {
    let mut socket = std::io::TcpStream::connect(server).unwrap();
    let http_req_status = socket.write(request);

    match http_req_status {
        Ok(t) => t,
        Err(e) => panic!(e),
    }

    let response = match socket.read_to_end() {
        Ok(t) => t,
        Err(e) => panic!(e),
    };

    let structure = match str::from_utf8(response.as_slice()) {
        Some(t) => t,
        None => panic!("Invalid UTF-8 sequence"),
    };

    let http_pieces = structure.split_str("\n");
    let parts: Vec<&str> = http_pieces.collect();
    parts[parts.len() - 1].to_string()
}

