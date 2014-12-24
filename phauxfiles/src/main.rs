extern crate serialize;
use serialize::json;
use std::fmt;
use std::str;

#[deriving(Decodable, Encodable)]
pub struct FauxPerson {
    name: String,
    surname: String,
    gender: String,
    country: String,
}

impl fmt::Show for FauxPerson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}\n{} from {}", self.name, self.surname, self.gender, self.country)
    }
}

#[deriving(Decodable, Encodable)]
pub struct ImageUrl {
    epic: String,
    bigger: String,
    normal: String,
    mini: String,
}

#[deriving(Decodable, Encodable)]
pub struct FaceCollection {
    username: String,
    image_urls: ImageUrl,
}

impl fmt::Show for FaceCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.image_urls.epic)
    }
}

fn main() {
    let names = http_get("api.uinames.com:80", b"GET http://api.uinames.com/?amount=6 HTTP/1.0\n\n");
    let people: Vec<FauxPerson> = json::decode(names.as_slice()).unwrap();
    for who in people.iter() {
        let faces = http_get("uifaces.com:80", b"GET /api/v1/random HTTP/1.0\nHost: uifaces.com\n\n");
        let urls: FaceCollection = json::decode(faces.as_slice()).unwrap();
        println!("{}\n{}\n", who.to_string(), urls.to_string());
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

