extern crate getopts;
extern crate serialize;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use serialize::json;
use std::os;
use std::str;
use fauxperson::{FauxPerson,FaceCollection};

mod fauxperson;
mod outfile;

pub struct Arguments {
    program_name: String,
    entries: String,
    filename: String,
    exit: bool,
}

fn main() {
    let args: Vec<String> = os::args();
    let opts = parse_args(args);

    if opts.exit {
        return;
    }

    generate_page(opts.filename, opts.entries);
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

fn generate_page(outfile_name: String, count: String) {
    let mut out = outfile::FileIo::new(outfile_name);

    let path = format!("/?amount={}", count);
    let names = http_get("api.uinames.com", 80, path.as_slice());
    let people: Vec<FauxPerson> = json::decode(names.as_slice()).unwrap();
    out.write("<html><head><title>Fake Search Results</title>");
    out.write("<meta http-equiv='Content-Type' content='text/html; charset=utf-8' />");
    out.write("<link rel='stylesheet' href='format.css'");
    out.write("</head><body>");
    for who in people.iter() {
        let faces = http_get("uifaces.com", 80, "/api/v1/random");
        let urls: FaceCollection = json::decode(faces.as_slice()).unwrap();
        let div = format!("<div class='profile'>\n{}\n{}\n</div>\n", urls.to_string(), who.to_string());
        out.write(div.as_slice());
    }
    out.write("</body></html>");
}

fn parse_args(arguments: Vec<String>) -> Arguments {
    let opts = &[
        optopt("n", "number-of-entries", "set output file name", "COUNT"),
        optopt("o", "output-file", "set output file name", "NAME"),
        optflag("h", "help", "print this help menu")
    ];

    let matches = match getopts(arguments.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut args = Arguments {
        program_name: arguments[0].clone(),
        entries: "".to_string(),
        filename: "".to_string(),
        exit: matches.opt_present("h"),
    };

    if args.exit {
        print_usage(args.program_name.as_slice(), opts);
    }

    args.entries = match matches.opt_str("n") {
        Some(x) => x.clone(),
        None => "6".to_string(),
    };

    let output = matches.opt_str("o");
    args.filename = match output {
        Some(x) => x,
        None => "".to_string(),
    };

    args
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

