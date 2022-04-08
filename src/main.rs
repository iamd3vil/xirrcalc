use std::fs::File;
use std::{process, ffi::OsString};
use std::env;
use std::error::Error;
use xirr::*;

fn main() {
    if let Err(err) = run() {
        println!("error reading csv: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut pmnts: Vec<Payment> = vec![];
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let payment = Payment{
            amount: record[0].parse().unwrap(),
            date: record[1].parse().unwrap()
        };
        pmnts.push(payment);
    }
    let xirr = compute(&pmnts).unwrap();
    println!("xirr is: {}%", xirr * 100.0);
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}