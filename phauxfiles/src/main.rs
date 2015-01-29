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

mod fauxperson;
mod outfile;
mod absurl;

pub struct Arguments {
    program_name: String,
    entries: Option<i16>,
    filename: Option<String>,
    port: Option<u16>,
    exit: bool,
}

fn main() {
    let args: Vec<String> = os::args();
    let opts = parse_args(args);

    if opts.exit {
        return;
    }

    match opts.port {
        Some(p) => serve_http(p, opts.entries),
        None => generate_page(opts.filename, opts.entries),
    }
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

fn generate_page(outfile_name: Option<String>, count: Option<i16>) {
    let mut out = outfile::FileIo::new(match outfile_name {
        Some(n) => n,
        None => "".to_string(),
    });
    let response = generate_page_text(count);
    out.write(response.as_slice());
}

fn generate_page_text(count: Option<i16>) -> String {
    let path = format!("/?amount={}", match count {
        Some(c) => c,
        None => 6i16,
    });
    let names = http_get("api.uinames.com", 80, path.as_slice());
    let people: Vec<FauxPerson> = json::decode(names.as_slice()).unwrap();
    let html_a = "<!DOCTYPE html>\n<html>\n<head><title>Fake Search Results</title>";
    let html_b = "<meta http-equiv='Content-Type' content='text/html; charset=utf-8' />";
    let html_c = "<link rel='stylesheet' href='format.css'>";
    let html_d = "</head><body>";
    let mut html = format!("{}\n{}\n{}\n{}\n", html_a, html_b, html_c, html_d);
    for who in people.iter() {
        let faces = http_get("uifaces.com", 80, "/api/v1/random");
        let urls: FaceCollection = json::decode(faces.as_slice()).unwrap();
        let div = format!("<div class='profile'>\n{}\n{}\n</div>\n", urls.to_string(), who.to_string());
        html = format!("{}{}\n", html, div);
    }
    let html_e = "</body></html>";
    html = format!("{}{}", html, html_e);

    html
}

fn parse_args(arguments: Vec<String>) -> Arguments {
    let opts = &[
        optopt("n", "number-of-entries", "set output file name", "COUNT"),
        optopt("o", "output-file", "set output file name", "NAME"),
        optopt("s", "server-port", "run a web server", "SERVE"),
        optflag("h", "help", "print this help menu")
    ];

    let matches = match getopts(arguments.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut args = Arguments {
        program_name: arguments[0].clone(),
        entries: None,
        filename: None,
        port: None,
        exit: matches.opt_present("h"),
    };

    if args.exit {
        print_usage(args.program_name.as_slice(), opts);
    }

    args.entries = match matches.opt_str("n") {
        Some(x) => x.as_slice().parse(),
        None => None,
    };

    args.filename = matches.opt_str("o");

    args.port = match matches.opt_str("s") {
        Some(x) => x.as_slice().parse(),
        None => None,
    };

    args
}

fn return_page(req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref p) => {
            let url = absurl::AbsUrl::new(p);
            let count = url.get("count".to_string(), "6".to_string());

            match (&req.method, url.path.as_slice()) {
                (&Get, "/") => {
                    let html = generate_page_text(count.parse());
                    let out = html.as_bytes();
                    res.headers_mut().set(ContentLength(out.len() as u64));
                    let mut res = res.start();
                    res.write(out).unwrap();
                    res.unwrap().end().unwrap();
                    return;
                },
                (&Get, "/format.css") => {
                    let path = Path::new("format.css");
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

