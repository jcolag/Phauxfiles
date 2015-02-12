use getopts::{optopt,optflag,getopts,OptGroup,usage};

pub struct Arguments {
    pub program_name: String,
    pub entries: Option<i16>,
    pub filename: Option<String>,
    pub port: Option<u16>,
    pub exit: bool,
}

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

pub fn parse_args(arguments: Vec<String>) -> Arguments {
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

