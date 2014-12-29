extern crate serialize;
use serialize::json;
use std::os;
use std::str;
use fauxperson::{FauxPerson,FaceCollection};

mod fauxperson;

fn main() {
    let args = os::args();
    let mut count = "6";
    if args.len() > 1 {
        count = args[1].as_slice();
    }
    let path = format!("/?amount={}", count);
    let names = http_get("api.uinames.com", 80, path.as_slice());
    let people: Vec<FauxPerson> = json::decode(names.as_slice()).unwrap();
    println!("<html><head><title>Fake Search Results</title>");
    println!("<meta http-equiv='Content-Type' content='text/html; charset=utf-8' />");
    println!("<link rel='stylesheet' href='format.css'");
    println!("</head><body>");
    for who in people.iter() {
        let faces = http_get("uifaces.com", 80, "/api/v1/random");
        let urls: FaceCollection = json::decode(faces.as_slice()).unwrap();
        println!("<div class='profile'>\n{}\n{}\n</div>\n", urls.to_string(), who.to_string());
    }
    println!("</body></html>")
}

fn http_get(host: &str, port: i32, path: &str) -> String {
    let server = format!("{}:{}", host, port.to_string());
    let request = format!("GET {} HTTP/1.0\nHost: {}\n\n", path.to_string(), host);
    let mut socket = std::io::TcpStream::connect(server.as_slice()).unwrap();
    let http_req_status = socket.write(request.as_bytes());

    match http_req_status {
        Ok(t) => t,
        Err(e) => panic!(e),
    }

    let response = match socket.read_to_end() {
        Ok(t) => t,
        Err(e) => panic!(e),
    };

    let structure = match str::from_utf8(response.as_slice()) {
        Ok(t) => t,
        Err(e) => panic!("Invalid UTF-8 sequence, {}", e),
    };

    let http_pieces = structure.split_str("\n");
    let parts: Vec<&str> = http_pieces.collect();
    parts[parts.len() - 1].to_string()
}

