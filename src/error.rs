use thiserror::Error;
use std::io;

pub const ERR_READ_MSG: &str = "I\\O error while reading from data source";
pub const ERR_WRITE_MSG: &str = "I\\O error while writing to data source";

pub const ERR_FORMAT: &str = "Input or output format is incorrect. Use txt, csv or bin.";

pub const ERR_PARAMS_CONVERTER: &str = "Invalid args. Try to use:\n <input-filename> txt|csv|bin txt|csv|bin <output-filename>";
pub const ERR_PARAMS_COMPARER: &str = "Invalid args. Try to use:\n <first-filename> txt|csv|bin txt|csv|bin <second-filename>";


#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Input-output error: {0}")]
    Io(#[from] io::Error),

    #[error("Parsing string to JSON error: {0}")]
    JSON(#[from] serde_json::Error),

    #[error("Parsing value error in transaction's CSV-string # {0}. String structure is incorrect")]
    InvalidCSVStructure(usize),

    #[error("Parsing value error in TXT-string {0}. String structure is incorrect")]
    InvalidTxtStrStructure(String),

    #[error("Parsing value error in transaction #{tx_numb}")]
    InvalidValue {tx_numb: usize},

    #[error("Input or output format is incorrect. Use txt, csv or bin.")]
    InvalidFormat,

    #[error("Unknown parsing error")]
    Unknown,
}