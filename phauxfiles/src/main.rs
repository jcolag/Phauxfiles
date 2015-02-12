extern crate getopts;
extern crate "rustc-serialize" as rustc_serialize;
extern crate hyper;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use hyper::{Client, Get};
use hyper::header::common::ContentLength;
use hyper::server::{Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use rustc_serialize::json;
use std::collections::HashMap;
use std::io::File;
use std::io::net::ip::Ipv4Addr;
use std::os;
use fauxperson::{FauxPerson,FaceCollection};
use validate::validator;

mod fauxperson;
mod outfile;
mod absurl;
mod validate;
mod args;

fn main() {
    let args: Vec<String> = os::args();
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
    out.write(response.as_slice());
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
    let names = http_get("api.uinames.com", 80, path.as_slice());
    let people: Vec<FauxPerson> = json::decode(names.as_slice()).unwrap();
    let html_a = "<!DOCTYPE html>\n<html>\n<head><title>Fake Search Results</title>";
    let html_b = "<meta http-equiv='Content-Type' content='text/html; charset=utf-8' />";
    let html_c = "<link rel='stylesheet' href='format.css'>";
    let html_d = "<link rel='shortcut icon' href='favicon.ico' />";
    let html_e = "</head><body>";
    let mut html = format!("{}\n{}\n{}\n{}\n{}\n", html_a, html_b, html_c, html_d, html_e);
    for who in people.iter() {
        let faces = http_get("uifaces.com", 80, "/api/v1/random");
        let urls: FaceCollection = json::decode(faces.as_slice()).unwrap();
        let div = format!("<div class='profile'>\n{}\n{}\n</div>\n", urls.to_string(), who.to_string());
        html = format!("{}{}\n", html, div);
    }
    let html_f = "</body></html>";
    html = format!("{}{}", html, html_f);

    html
}

fn serve_file(mut res: Response, name: &str) {
    let path = Path::new(name);
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => { return; },
    };
        let css = match file.read_to_string() {
        Ok(s) => s,
        Err(_) => { return; },
    };
    let out = css.as_bytes();
    res.headers_mut().set(ContentLength(out.len() as u64));
    let mut res = res.start();
    res.write(out).unwrap();
    res.unwrap().end().unwrap();
}

fn return_page(req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref p) => {
            let url = absurl::AbsUrl::new(p);
            let count = url.get("count".to_string(), Some("6".to_string())).unwrap();
            let nation = validator::country(url.get("where".to_string(), None));
            let sex = validator::gender(url.get("sex".to_string(), None));

            match (&req.method, url.path.as_slice()) {
                (&Get, "/") => {
                    let html = generate_page_text(count.parse(), nation, sex);
                    let out = html.as_bytes();
                    res.headers_mut().set(ContentLength(out.len() as u64));
                    let mut res = res.start();
                    res.write(out).unwrap();
                    res.unwrap().end().unwrap();
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
    let server = Server::http(Ipv4Addr(127, 0, 0, 1), port);
    let mut listening = server.listen(return_page).unwrap();
    listening.await();
}

fn http_get(host: &str, port: i32, path: &str) -> String {
    let url = format!("http://{}:{}{}", host, port, path);
    let mut client = Client::new();
    let res = client.get(url.as_slice()).send();
    let mut response = match res {
        Ok(x) => x,
        Err(e) => panic!(e),
    };
    response.read_to_string().unwrap()
}
