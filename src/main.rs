use overcomplicating_things::api::*;



use std::io::Cursor;
use clap::Parser;

use std::io;

#[derive(Parser,Default,Debug)]
struct Arguments {
    #[arg(default_value_t = String::new())]
    ytid: String,
}

fn main() {
    let mut args = Arguments::parse();
    
    if args.ytid.is_empty() {
        println!("Provide JUST the Youtube ID, the part that comes after the =, then hit Enter.");
        io::stdin()
        .read_line(&mut args.ytid)
        .expect("Failed to read line");
    }

    let x = args.ytid;

    let dl = submit_and_download(x.to_string(), "song".to_string());

    let mut file = std::fs::File::create("./".to_owned() + &x.clone() + ".zip").unwrap();

    let mut content =  Cursor::new(dl.unwrap().bytes().unwrap());

    std::io::copy(&mut content, &mut file).unwrap();
}
