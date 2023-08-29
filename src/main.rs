use chrono::NaiveDate;
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;
use xirr::Payment;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let pmnts = read_payments_from_file(&file_path)?;
    let xirr = xirr::compute(&pmnts)?;
    println!("xirr is: {:.2}%", xirr * 100.0);
    Ok(())
}

fn get_file_path() -> Result<String, Box<dyn Error>> {
    env::args()
        .nth(1)
        .ok_or_else(|| From::from("expected 1 argument, but got none"))
}

fn read_payments_from_file(file_path: &str) -> Result<Vec<Payment>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let dfmts = get_dfmts();
    let mut pmnts = vec![];

    for result in rdr.records() {
        let record = result?;
        let date = get_date(&record[1], &dfmts)?;
        let amount = record[0].parse::<f64>()?;
        pmnts.push(Payment { amount, date });
        println!("payment: {}, date: {}", amount, date);
    }

    Ok(pmnts)
}

fn get_dfmts() -> Vec<String> {
    match env::args().nth(2) {
        Some(dfmt) => vec![dfmt],
        None => vec![
            "%d/%m/%y".to_string(),
            "%d/%m/%Y".to_string(),
            "%Y-%m-%d".to_string(),
        ],
    }
}

fn get_date(date: &str, dfmts: &[String]) -> Result<NaiveDate, Box<dyn Error>> {
    for fmt in dfmts {
        if let Ok(parsed_date) = NaiveDate::parse_from_str(date, fmt) {
            return Ok(parsed_date);
        }
    }
    Err(From::from("invalid date"))
}
