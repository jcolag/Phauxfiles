/*
#![feature(env)]
#![feature(fs)]
#![feature(net)]
#![feature(path)]
#![feature(io)]
*/
extern crate getopts;
extern crate rustc_serialize;
extern crate hyper;
use hyper::{Client, Get};
use hyper::header::ContentLength;
use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use rustc_serialize::json;
use std::fs::File;
use std::path::Path;
use std::io::{Read,Write};
//use std::net::IpAddr;
use fauxperson::{FauxPerson,FaceCollection};
use validate::validator;

mod fauxperson;
mod outfile;
mod absurl;
mod validate;
mod args;

fn main() {
    let args: std::env::Args = std::env::args();
    let opts = args::parse_args(args);

    if opts.exit {
        return;
    }

    match opts.port {
        Some(p) => serve_http(p, opts.entries),
        None => generate_page(opts.filename, opts.entries),
    }
}

fn generate_page(outfile_name: Option<String>, count: Option<i16>) {
    let mut out = outfile::FileIo::new(match outfile_name {
        Some(n) => n,
        None => "".to_string(),
    });
    let response = generate_page_text(count, None, None);
    out.write(&*response);
}

fn generate_page_text(count: Option<i16>, country: Option<String>, sex: Option<String>) -> String {
    let path = format!("/?amount={}{}{}", match count {
        Some(c) => c,
        None => 6i16,
    }, match country {
        Some(c) => format!("&country={}", c),
        None => "".to_string(),
    }, match sex {
        Some(s) => format!("&gender={}", s),
        None => "".to_string(),
    });
    let names = http_get("api.uinames.com", 80, &*path);
    let people: Vec<FauxPerson> = json::decode(&*names).unwrap();
    let path = Path::new("template.html");
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => { return "".to_string(); },
    };
    let mut html = String::new();
    match file.read_to_string(&mut html) {
        Ok(_) => 1,
        Err(_) => { return "".to_string(); },
    };
    for who in people.iter() {
        let faces = http_get("uifaces.com", 80, "/api/v1/random");
        let urls: FaceCollection = json::decode(&*faces).unwrap();
        let div = format!("<div class='profile'>\n<a href='http://uifaces.com/{}'>{}</a>\n{}\n</div>\n", urls.username, urls.to_string(), who.to_string());
        html.push_str(&*div);
        html.push_str("\n");
    }
    html.push_str("</body></html>");
    html
}

fn serve_file(mut res: Response, name: &str) {
    let path = Path::new(name);
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => { return; },
    };
    let mut css = String::new();
    match file.read_to_string(&mut css) {
        Ok(_) => 1,
        Err(_) => { return; },
    };
    let out = css.as_bytes();
    res.headers_mut().set(ContentLength(out.len() as u64));
    match res.start() {
        Ok(mut r) => {
            r.write(out).unwrap();
            r.end().unwrap();
        },
        Err(_) => (),
    };
}

fn return_page(req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref p) => {
            let url = absurl::AbsUrl::new(p);
            let count_str = url.get("count".to_string(), Some("6".to_string())).unwrap();
            let count: Option<i16> = match count_str.parse() {
                Ok(x) => Some(x),
                Err(_) => None,
            };
            let nation = validator::country(url.get("where".to_string(), None));
            let sex = validator::gender(url.get("sex".to_string(), None));

            match (&req.method, &*url.path) {
                (&Get, "/") => {
                    let html = generate_page_text(count, nation, sex);
                    let out = html.as_bytes();
                    res.headers_mut().set(ContentLength(out.len() as u64));
                    match res.start() {
                        Ok(mut r) => {
                            r.write(out).unwrap();
                            r.end().unwrap();
                        },
                        Err(_) => (),
                    };
                    return;
                },
                (&Get, "/favicon.ico") => {
                    serve_file(res, "favicon.ico");
                    return;
                },
                (&Get, "/format.css") => {
                    serve_file(res, "format.css");
                    return;
                },
                _ => {
                    *res.status_mut() = hyper::NotFound;
                    res.start().and_then(|res| res.end()).unwrap();
                    return;
                },
            }
        },
        _ => {
            res.start().and_then(|res| res.end()).unwrap();
            return;
        },
    }
}

fn serve_http(port: u16, count: Option<i16>) {
    let url = format!("127.0.0.1:{}", port);
    Server::http(&*url).unwrap().handle(return_page).unwrap();
}

fn http_get(host: &str, port: i32, path: &str) -> String {
    let url = format!("http://{}:{}{}", host, port, path);
    let client = Client::new();
    let res = client.get(&*url).send();
    let mut response = match res {
        Ok(x) => x,
        Err(e) => panic!(e),
    };
    let mut resp = String::new();
    match response.read_to_string(&mut resp) {
      Ok(_) => 1,
      Err(_) => { return "".to_string(); },
    };
    resp.clone()
}
