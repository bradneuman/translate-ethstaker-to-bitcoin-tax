use std::{error::Error, io};
// use serde::Deserialize;

use fixed::types::I8F56;
type Fixed = I8F56;

/// For rocketpool there are additional columns but the initial colums are all the same.
const SHARED_COLUMN_NAMES: [&str; 5] = [
    "Date",
    "Validator Index",
    "Price [USD/ETH]",
    "Consensus Layer Income [ETH]",
    "Execution Layer Income [ETH]",
];
const ROCKETPOOL_EXTRA_COLUMN_NAMES: [&str; 4] = [
    "Node Address",
    "Smoothing Pool Income [ETH]",
    "Rocket Pool Node Income [RPL]",
    "Price [USD/RPL]",
];

#[derive(Debug, serde::Deserialize)]
struct InputRow {
    // TODO:(bn) Date type using something like time::serde::iso8601
    // TODO:(bn) figure out what timezone ethstaker.tax is actually using
    #[serde(rename = "Date")]
    date: String,

    // TODO:(bn) fixed point numbers
    // TODO:(bn) strong types?
    #[serde(rename = "Consensus Layer Income [ETH]")]
    consensus_income_eth: Option<Fixed>,

    #[serde(rename = "Execution Layer Income [ETH]")]
    execution_income_eth: Option<Fixed>,

    #[serde(rename = "Smoothing Pool Income [ETH]")]
    smoothing_pool_income_eth: Option<Fixed>,

    #[serde(rename = "Rocket Pool Node Income [RPL]")]
    node_income_rpl: Option<Fixed>,
}

#[derive(Debug)]
enum Translator {
    Eth(Fixed),
    Rpl(Fixed),
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct OutputRow {
    date: String,
    action: String,
    account: String,
    symbol: String,
    volume: Fixed,
}

impl Translator {
    // TODO:(bn) return iterator instead of vector
    pub fn from(input: &InputRow) -> Vec<Self> {
        let mut ret = Vec::new();

        let solo = match (input.consensus_income_eth, input.execution_income_eth) {
            (Some(a), Some(b)) => Some(a + b),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        if let Some(solo_eth) = solo {
            ret.push(Translator::Eth(solo_eth));
        }

        if let Some(smoothing_eth) = input.smoothing_pool_income_eth {
            ret.push(Translator::Eth(smoothing_eth));
        }

        if let Some(rpl) = input.node_income_rpl {
            ret.push(Translator::Rpl(rpl));
        }

        ret
    }

    pub fn to(&self, date: &str, account: &str) -> OutputRow {
        match &self {
            Self::Eth(volume) => {
                OutputRow {
                    date: date.to_string(),
                    account: account.to_string(),
                    // TODO:(bn) use a const somehow?
                    action: String::from("MINING"),
                    symbol: String::from("ETH"),
                    volume: *volume,
                }
            }
            Self::Rpl(volume) => {
                OutputRow {
                    date: date.to_string(),
                    account: account.to_string(),
                    // TODO:(bn) use a const somehow?
                    action: String::from("MINING"),
                    symbol: String::from("RPL"),
                    volume: *volume,
                }
            }
        }
    }
}

fn translate() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(io::stdin());

    let mut writer = csv::Writer::from_writer(io::stdout());

    let source;

    {
        let headers = reader.headers()?;

        // TODO:(bn) errors instead of panic?
        const SOLO_NUM_COLUMNS: usize = SHARED_COLUMN_NAMES.len();
        const RPL_NUM_COLUMNS: usize =
            SHARED_COLUMN_NAMES.len() + ROCKETPOOL_EXTRA_COLUMN_NAMES.len();

        source = match headers.len() {
            SOLO_NUM_COLUMNS => "Solo Staking",
            RPL_NUM_COLUMNS => "Rocketpool Staking",
            n => panic!("Invalid number of columns: {n}"),
        };

        for (i, v) in headers.iter().enumerate() {
            if i < SHARED_COLUMN_NAMES.len() {
                assert_eq!(v, SHARED_COLUMN_NAMES[i]);
            } else {
                let i = i - SHARED_COLUMN_NAMES.len();
                assert_eq!(v, ROCKETPOOL_EXTRA_COLUMN_NAMES[i]);
            }
        }
    }

    for result in reader.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let input: InputRow = result?;
        // println!("{:?}", input);

        for t in Translator::from(&input) {
            // println!("  ** {:?}", t);
            let output = t.to(&input.date, source);
            // println!("  -> {:?}", output);

            writer.serialize(output)?;
        }

        // println!("--------------------------------------------------------------------------------");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    translate()
}
