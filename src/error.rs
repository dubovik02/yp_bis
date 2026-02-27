use thiserror::Error;
use std::io;

/// Сообщение об ошибке чтения
pub const ERR_READ_MSG: &str = "I\\O error while reading from data source";
/// Сообщение об ошибке записи
pub const ERR_WRITE_MSG: &str = "I\\O error while writing to data source";
/// Сообщение об ошибке формата
pub const ERR_FORMAT: &str = "Input or output format is incorrect. Use txt, csv or bin.";
/// Сообщение об ошибке указания параметров при запуске утилиты ковертера
pub const ERR_PARAMS_CONVERTER: &str = "Invalid args. Try to use:\n <input-filename> txt|csv|bin txt|csv|bin <output-filename>";
/// Сообщение об ошибке указания параметров при запуске утилиты сравнения
pub const ERR_PARAMS_COMPARER: &str = "Invalid args. Try to use:\n <first-filename> txt|csv|bin txt|csv|bin <second-filename>";


/// Ошибки парсинга
#[derive(Error, Debug)]
pub enum ParserError {
    /// Ошибки ввода-вывода
    #[error("Input-output error: {0}")]
    Io(#[from] io::Error),
    /// Ошибки парсинга строк JSON
    #[error("Parsing string to JSON error: {0}")]
    JSON(#[from] serde_json::Error),
    /// Ошибки парсинга CSV формата
    #[error("Parsing value error in transaction's CSV-string # {0}. String structure is incorrect")]
    InvalidCSVStructure(usize),
    /// Ошибки парсинга TXT формата
    #[error("Parsing value error in TXT-string {0}. String structure is incorrect")]
    InvalidTxtStrStructure(String),
    /// Ошибки парсинга транзакции
    #[error("Parsing value error in transaction #{tx_numb}")]
    InvalidValue {
        /// tx_numb: usize - номер транзакции в наборе
        tx_numb: usize
    },
    /// Ошибки не соответствия формата
    #[error("Input or output format is incorrect. Use txt, csv or bin.")]
    InvalidFormat,
    /// Иные ошибки
    #[error("Unknown parsing error")]
    Unknown,
}