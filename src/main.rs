use std::{error::Error, io};
// use serde::Deserialize;

use fixed::types::I8F24;
type Fixed = I8F24;

/// For rocketpool there are additional columns but the initial colums are all the same.
const SHARED_COLUMN_NAMES: [&'static str; 5] = [
    "Date",
    "Validator Index",
    "Price [USD/ETH]",
    "Consensus Layer Income [ETH]",
    "Execution Layer Income [ETH]",
];
const ROCKETPOOL_EXTRA_COLUMN_NAMES: [&'static str; 4] = [
    "Node Address",
    "Smoothing Pool Income [ETH]",
    "Rocket Pool Node Income [RPL]",
    "Price [USD/RPL]",
];

#[derive(Debug, serde::Deserialize)]
struct Row {
    // TODO:(bn) Date type
    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Validator Index")]
    validator_index: Option<u64>,

    // TODO:(bn) fixed point numbers
    // TODO:(bn) strong types?
    #[serde(rename = "Consensus Layer Income [ETH]")]
    consensus_income_eth: Option<Fixed>,

    #[serde(rename = "Execution Layer Income [ETH]")]
    execution_income_eth: Option<Fixed>,

    #[serde(rename = "Smoothing Pool Income [ETH]")]
    smothing_pool_income_eth: Option<Fixed>,

    #[serde(rename = "Rocket Pool Node Income [RPL]")]
    node_income_rpl: Option<Fixed>,
}

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(io::stdin());

    {
        let headers = rdr.headers()?;

        // TODO:(bn) errors instead of panic?
        let solo_num_columns = SHARED_COLUMN_NAMES.len();
        let rpl_num_columns = SHARED_COLUMN_NAMES.len() + ROCKETPOOL_EXTRA_COLUMN_NAMES.len();

        assert!(
            headers.len() == solo_num_columns || headers.len() == rpl_num_columns,
            "Invalid number of columns"
        );
        for (i, v) in headers.iter().enumerate() {
            if i < SHARED_COLUMN_NAMES.len() {
                assert_eq!(v, SHARED_COLUMN_NAMES[i]);
            } else {
                let i = i - SHARED_COLUMN_NAMES.len();
                assert_eq!(v, ROCKETPOOL_EXTRA_COLUMN_NAMES[i]);
            }
        }
    }

    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record: Row = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    example()
}
