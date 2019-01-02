
extern crate mailparse;
use mailparse::*;
    
#[macro_use]
extern crate structopt;

extern crate regex;

use std::env;
use std::io::prelude::*;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use regex::RegexBuilder;


const MAIL_SEPARATOR : &str = "-- END OF MAIL --";

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {

    /// Pattern that should be searched
    #[structopt(long = "pattern")]
    pattern: String,


    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,


}

struct Conf {
    opt : Opt,

    // precompiled pattern
    pattern : regex::Regex,
}

fn check(mut reader : Box<BufRead>, conf : &Conf) {
    let mut content = String::new();
    reader.read_to_string(&mut content);

    let parsed = parse_mail(content.as_bytes()).unwrap();

    let mut found = false;

    let body = parsed.get_body().unwrap();
    if let Some(x) = conf.pattern.find(&body) {
        println!("found");
        found = true;
    }   

    if found {
        println!("{}", content);
        println!("\n\n{}\n\n", MAIL_SEPARATOR);
    }
}


fn main() {

    let opt = Opt::from_args();
    println!("{:?}", opt);

    let pattern = RegexBuilder::new(&opt.pattern)
        .case_insensitive(true)
        .build()
        .expect("Invalid Regex");


    let conf = Conf {
        opt,
        pattern,
    };



    if conf.opt.files.len() == 0 {
        // read from stdin
        let reader = Box::new(BufReader::new(io::stdin()));
        check(reader, &conf);
    }
    else {

        for i in &conf.opt.files {
            let mut file = File::open(i).expect("file not found");
            let reader = Box::new(BufReader::new(file));
            check(reader, &conf);
        }

    }
}
