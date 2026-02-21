use std::{collections::HashSet, fs::File, path::Path};

use serde::{Serialize, Deserialize};
use serde_json::Result;
use strum_macros::Display;

pub const CVS_HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n";
pub const MAGIC: &str = "YPBN";
pub const BIN_BODY_LEN: u32 = 46;

pub mod txt_format;
pub mod csv_format;
pub mod bin_format;
pub mod error;

#[derive(Display, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd)]
pub enum TransactionType {
   DEPOSIT = 0, 
   TRANSFER = 1, 
   WITHDRAWAL = 2,
   EMPTY = 3,
}

#[derive(Display, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd)]
pub enum TransactionStatus {
   SUCCESS = 0,
   FAILURE = 1,
   PENDING = 2,
   EMPTY = 3,
}

#[derive(Display, PartialEq, Debug, Eq)]
pub enum TransactionsFormatType {
    TXT = 0,
    CSV = 1,
    BIN = 2,
    UNKNOWN = 3,
}

pub enum FormatParsers {
    Txt,
    Csv,
    Bin,
}

#[derive(Debug)]
pub struct CliParams {
    pub first_file_name: String,
    pub first_file_format: TransactionsFormatType,
    pub second_file_name: String,
    pub second_file_format: TransactionsFormatType,
}

impl CliParams {
    pub fn new() -> Self {
        Self {
            first_file_name: String::new(),
            first_file_format: TransactionsFormatType::UNKNOWN,
            second_file_name: String::new(),
            second_file_format: TransactionsFormatType::UNKNOWN,
        }
    }
}

impl Default for CliParams {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionsParser for FormatParsers {
    fn get_using_format_type(&self) -> TransactionsFormatType {
       match self {
            Self::Txt => txt_format::TxtParser::default().get_using_format_type(),
            Self::Csv => csv_format::CsvParser::default().get_using_format_type(),
            Self::Bin => bin_format::BinParser::default().get_using_format_type(),
       }
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>> {
        match self {
            Self::Txt => txt_format::TxtParser::default().from_read(source),
            Self::Csv => csv_format::CsvParser::default().from_read(source),
            Self::Bin => bin_format::BinParser::default().from_read(source),
       }
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: Vec<Transaction>) -> Result<()> {
        match self {
            Self::Txt => txt_format::TxtParser::default().write_to(target, data),
            Self::Csv => csv_format::CsvParser::default().write_to(target, data),
            Self::Bin => bin_format::BinParser::default().write_to(target, data),
       }
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub struct Transaction {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub description: String, 
}

impl Transaction {
    pub fn new() -> Self {
        Self {
            tx_id: 0,
            tx_type: TransactionType::EMPTY,
            from_user_id: 0,
            to_user_id: 0,
            amount: 0,
            timestamp: 0,
            status: TransactionStatus::EMPTY,
            description: String::new(),  
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}


/// Трейт функциональности парсера данных из формата
pub trait TransactionsParser {

    /// Возвращает тип формата транзакций - txt, csv, bin
    fn get_using_format_type(&self) -> TransactionsFormatType;

    /// Функция чтения из источника представленного в конкретном формате финансовых данных
    /// * 'source' - источник даннвх (файл, буфер)
    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>>;

    /// Функция записи в источник в конкретном формате финансовых данных
    /// * 'target &mut W,' - получатель даннвх (файл, буфер), * 'data Vec<Transaction>' - данных о финансовых транзакциях
    fn write_to<W: std::io::Write>(&self, target: &mut W, data: Vec<Transaction>) -> Result<()>;
}

pub fn txt_to_json_str(tx: &str) -> String {
    let tx_json = &tx.replace(": ", ":");

    let json_vec: Vec<&str> = tx_json.split(":").into_iter().collect();

    let type_val; 
    let status_val; 

    let key_str = "\"".to_owned() + json_vec[0].to_ascii_lowercase().as_str() + "\"" + ":";

    if json_vec[0] == "TX_TYPE" {
        type_val = json_vec[1];
        return key_str + "\"" + type_val.to_string().as_str() + "\"";
    };

    if json_vec[0] == "STATUS" {
        status_val = json_vec[1];
        return key_str + "\"" + status_val.to_string().as_str() + "\"";
    }  

    let key_val = "".to_owned() + json_vec[1];
    key_str + key_val.as_str()
}

pub fn compare(tx_left: Transaction, tx_right: Transaction) -> bool {
    tx_left == tx_right
}

pub fn compare_tx_sets(left_side: Vec<Transaction>, right_side: Vec<Transaction>) -> bool {

    let mut result: bool = true;

    result = result && (left_side.len() == right_side.len());

    let left_hash: HashSet<Transaction> = left_side.into_iter().collect();
    let right_hash: HashSet<Transaction> = right_side.into_iter().collect();

    let left_difference: Vec<&Transaction> = left_hash.difference(&right_hash).collect();
    let right_difference: Vec<&Transaction> = right_hash.difference(&left_hash).collect();

    result = result && (left_difference.len() + right_difference.len() == 0);

    result
}

pub fn get_params(args: Vec<String>, is_file_creation: bool) -> CliParams {

    let mut params = CliParams::new();

    params.first_file_name = get_param_value(&args[1]);
    params.first_file_format = get_format_value(&args[2]);

    params.second_file_format = get_format_value(&args[3]);
    params.second_file_name = get_param_value(&args[4]);
    

    if params.first_file_format == TransactionsFormatType::UNKNOWN || 
        params.second_file_format == TransactionsFormatType::UNKNOWN {

        panic!("{}", error::ERR_FORMAT)
    }

    let mut file_ok: bool; 

    file_ok = Path::new(&params.first_file_name.trim()).exists();
    if !file_ok {
        panic!("File {} does not exist", params.first_file_name.trim());
    }

    if is_file_creation {
        file_ok = check_file_creation(&params.second_file_name);
        if !file_ok {
            panic!("File {} could'nt create", params.second_file_name.trim());
        }    
    } 
    else {
        file_ok = Path::new(&params.second_file_name.trim()).exists();
        if !file_ok {
            panic!("File {} does not exist", params.second_file_name.trim());
        }
    }
    params
}

fn check_file_creation(path: &str) -> bool {
    match File::create(Path::new(path.trim())) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn get_param_value(val: &String) -> String {
    val
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.contains("--"))
        .collect()
}

fn get_format_value(val: &String) -> TransactionsFormatType {
    match get_param_value(val).as_str() {
        "txt" => TransactionsFormatType::TXT,
        "csv" => TransactionsFormatType::CSV,
        "bin" => TransactionsFormatType::BIN,
        _ => TransactionsFormatType::UNKNOWN,
        
    }
}

pub fn get_parser_for_format(val: TransactionsFormatType) -> FormatParsers {
    match val {
        TransactionsFormatType::TXT => FormatParsers::Txt,
        TransactionsFormatType::CSV => FormatParsers::Csv,
        TransactionsFormatType::BIN => FormatParsers::Bin,
        _ => panic!("{}", error::ERR_FORMAT),
    }
}