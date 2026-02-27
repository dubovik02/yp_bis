//! Библиотека парсинга и сравнения данных о финансовых транзакция.

#![warn(missing_docs)] 
use std::{collections::HashSet, fs::File, path::Path};
use serde::{Serialize, Deserialize};
use strum_macros::Display;

use crate::error::ParserError;

/// Строка заголовка для формата CSV
pub const CVS_HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n";
/// Строковое значение маркера начала записи о транзакции для бинарного формата
pub const MAGIC: &str = "YPBN";
/// Базовая длина записи в байтах для формата bin
pub const BIN_BODY_LEN: u32 = 46;

/// Модуль реализации парсера текстового формата
pub mod txt_format;
/// Модуль реализации парсера формата csv
pub mod csv_format;
/// Модуль реализации парсера бинарного формата
pub mod bin_format;
/// Модуль декларации ошибок
pub mod error;

/// Типы транзакций
#[derive(Display, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd)]
pub enum TransactionType {
    /// - DEPOSIT – депозит
    DEPOSIT = 0, 
    /// - TRANSFER – перевод
    TRANSFER = 1, 
    /// - DEPOSIT – депозит
    WITHDRAWAL = 2,
    /// - EMPTY - не определен
    EMPTY = 3,
}

/// Статус транзакции
#[derive(Display, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd)]
pub enum TransactionStatus {
    /// - SUCCESS – успешное завершение
    SUCCESS = 0,
    /// - FAILURE – отказ
    FAILURE = 1,
    /// - PENDING – исполнение в процессе
    PENDING = 2,
    /// - EMPTY - не определен
    EMPTY = 3,
}

/// Типы форматов финансовых данных
#[derive(Display, PartialEq, Debug, Eq)]
pub enum TransactionsFormatType {
    /// - TXT – текстовый формат TXT
    TXT = 0,
    /// - CSV – формат с разделителем CSV
    CSV = 1,
    /// - BIN – бинарный формат BIN
    BIN = 2,
    /// - UNKNOWN - не определен
    UNKNOWN = 3,
}

/// Фабрика парсеров
pub enum FormatParsers {
    /// - Txt – возвражает парсер для текстового формата TXT
    Txt,
    /// - Csv – возвражает парсер для формата CSV
    Csv,
    /// - Bin – возвражает парсер для формата BIN
    Bin,
}

/// Структура данных о параметрах запуска утилит
#[derive(Debug)]
pub struct CliParams {
    /// - first_file_name – файл содержащий финансовые транзакции
    pub first_file_name: String,
    /// - first_file_format – формат файла <first_file_name> - csv, txt, bin.
    pub first_file_format: TransactionsFormatType,
    /// - second_file_name – файл содержащий финансовые транзакции
    pub second_file_name: String,
    /// - second_file_format – формат файла <second_file_name> - csv, txt, bin.
    pub second_file_format: TransactionsFormatType,
}

impl CliParams {
    /// Конструктор
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

/// Реализация TransactionsParser для фабрики парсеров
impl TransactionsParser for FormatParsers {
    fn get_using_format_type(&self) -> TransactionsFormatType {
       match self {
            Self::Txt => txt_format::TxtParser::default().get_using_format_type(),
            Self::Csv => csv_format::CsvParser::default().get_using_format_type(),
            Self::Bin => bin_format::BinParser::default().get_using_format_type(),
       }
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>, ParserError> {
        match self {
            Self::Txt => txt_format::TxtParser::default().from_read(source),
            Self::Csv => csv_format::CsvParser::default().from_read(source),
            Self::Bin => bin_format::BinParser::default().from_read(source),
       }
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError> {
        match self {
            Self::Txt => txt_format::TxtParser::default().write_to(target, data),
            Self::Csv => csv_format::CsvParser::default().write_to(target, data),
            Self::Bin => bin_format::BinParser::default().write_to(target, data),
       }
    }
}

/// Структура данных о финансовой транзакции
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub struct Transaction {
/// - tx_id – идентификатор транзакции.
    pub tx_id: u64,
/// - tx_type – тип транзакции: `DEPOSIT`, `TRANSFER`, или `WITHDRAWAL`.
    pub tx_type: TransactionType,
/// - from_user_id – идентификатор отправителя счета (используйте `0` для DEPOSIT).
    pub from_user_id: u64,
/// - to_usr_id – идентифицикатор получателя счета (используйте `0` для WITHDRAWAL).
    pub to_user_id: u64,
/// - account – сумма,
    pub amount: u64,
/// - timestamp – Unix epoch timestamp в миллисекундах.
    pub timestamp: u64,
/// - status – состояние транзакции: `SUCCESS`, `FAILURE`, или `PENDING`.
    pub status: TransactionStatus,
/// - description – произвольное текстовое описание, UTF-8 в двойныхкавычках.
    pub description: String, 
}

impl Transaction {
    /// Конструктор
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

    /// Возвращает тип формата транзакций которыми оперирует парсер - txt, csv, bin
    fn get_using_format_type(&self) -> TransactionsFormatType;

    /// Функция чтения из источника представленного в конкретном формате финансовых данных
    /// * source - источник данных содержащий транзакции и реализуйщий трейт Read (файл, буфер) в формате, обрабатываемом парсером
    fn from_read<Reader: std::io::Read>(&self, source: &mut Reader) -> Result<Vec<Transaction>, ParserError>;

    /// Функция записи набора транзакций в источник в конкретном формате финансовых данных
    /// * target - получатель данных реализуйщий трейт Write (файл, буфер), 
    /// * data - данные о финансовых транзакциях
    fn write_to<Writer: std::io::Write>(&self, target: &mut Writer, data: &Vec<Transaction>) -> Result<(), ParserError>;
}

fn txt_to_json_str(tx: &str) -> Result<String, ParserError> {
    let tx_json = &tx.replace(": ", ":");

    let json_vec: Vec<&str> = tx_json.split(":").collect();

    if json_vec.len() != 2 {
        return Result::Err(ParserError::InvalidTxtStrStructure(tx.to_string()));
    }

    let type_val; 
    let status_val; 

    let key_str = "\"".to_owned() + json_vec[0].to_ascii_lowercase().as_str() + "\"" + ":";

    if json_vec[0] == "TX_TYPE" {
        type_val = json_vec[1];
        return Ok(key_str + "\"" + type_val.to_string().as_str() + "\"");
    };

    if json_vec[0] == "STATUS" {
        status_val = json_vec[1];
        return Ok(key_str + "\"" + status_val.to_string().as_str() + "\"");
    }  

    let key_val = "".to_owned() + json_vec[1];
    Ok(key_str + key_val.as_str())
}

/// Функция сравнения двух транзакций
/// 
/// # Аргументы
/// 
/// * tx_left: Transaction - первая транзакция для сравнения
/// * tx_right: Transaction - втораЯ транзакция для сравнения
/// 
/// # Возвращаемое значение
/// Возвращает true, если значение всех полнй транзакции tx_left соответствуют полям транзакции tx_right,
/// иначе - false.
/// 
/// # Пример
/// ```ignore 
/// let result = compare(tx_left, tx_right);
/// if result {
///     println!("Tx's are the same");
/// }
/// else {
///     println!("Tx's are not the same");
/// }
/// ```
/// 
pub fn compare(tx_left: Transaction, tx_right: Transaction) -> bool {
    tx_left == tx_right
}


/// Функция сравнения двух наборов транзакций
/// 
/// # Аргументы
/// 
/// * left_side: Vec<Transaction> - первый набор транзакций
/// * left_side: Vec<Transaction> - второй набор транзакций
/// 
/// # Возвращаемое значение
/// Возвращает true, если:
/// * число транзакций содержащихся в наборах равно и
/// * разность набора left_side и набора right_side равна 0 элементов и
/// * разность набора right_side и набора left_side равна 0 элементов
/// 
/// иначе - false.
/// 
/// # Пример
/// ```ignore 
/// let tx: Transaction = Transaction { 
///      tx_id: (1), 
///      tx_type: (TransactionType::WITHDRAWAL), 
///      from_user_id: (0123456789), 
///      to_user_id: (9876543210), 
///      amount: (100), 
///      timestamp: (0), 
///      status: (TransactionStatus::SUCCESS), 
///      description: ("Transaction #1".to_owned())
/// };
/// let left_side: Vec<Transaction> = vec![tx];
/// let right_side: Vec<Transaction> = vec![tx];
/// let result = compare_tx_sets(left_side, right_side);
/// //true
/// ```
/// 
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

/// Функция получения парметров командной строки при запуске утилит
/// 
/// # Аргументы
/// 
/// * args: Vec<String> - набор аргументов запуска утилит
/// * is_file_creation: bool - флаг необходимости перезаписи (создания) файла 
///   при проверки доступности записи файла по пути указанному в параметрепроверки
/// 
/// # Возвращаемое значение
/// Экземпляр CliParams содержащий парметры запуска утилит
/// 
/// # Пример
/// ```ignore 
/// // CMD:> util file1.csv csv bin file2.bin
/// let args: Vec<String> = env::args().collect();
///
/// // pre-check 
/// if args.len() < 5 {
///     println!("{}", ERR_PARAMS_COMPARER);
///     return Ok(());
/// }
///
/// let params: CliParams = get_params(args, false);
/// println!("{}", params.first_file_name);
/// // file1.txt
/// ```
/// 
/// # Ошибки
/// panic - не поддерживаемые форматы, файл не существует или не может быть создан.
/// 
pub fn get_params(args: Vec<String>, is_file_creation: bool) -> CliParams {

    let mut params = CliParams::new();

    params.first_file_name = (&args[1]).to_owned();
    params.first_file_format = get_format_value(&args[2]);

    params.second_file_format = get_format_value(&args[3]);
    params.second_file_name = (&args[4]).to_owned();
    

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

fn get_format_value(val: &str) -> TransactionsFormatType {
    match val {
        "txt" => TransactionsFormatType::TXT,
        "csv" => TransactionsFormatType::CSV,
        "bin" => TransactionsFormatType::BIN,
        _ => TransactionsFormatType::UNKNOWN,
        
    }
}

/// Функция получение парсера данных для соответствующего формата
/// 
/// # Аргументы
/// 
/// * val: TransactionsFormatType - типп формата транзакций для которого необходимо получить парсер
/// 
/// # Возвращаемое значение
/// Экземпляр FormatParsers содержащий парсер для запрошенного формата
/// 
/// # Пример
/// ```ignore 
/// let parser = get_parser_for_format(TransactionsFormatType::BIN);
/// assert_eq!(parser.get_using_format_type(), TransactionsFormatType::BIN);
/// // парсер для работы с бинарным форматом
/// ```
/// 
/// # Ошибки
/// panic - парсер для формата не определен.
/// 
pub fn get_parser_for_format(val: TransactionsFormatType) -> FormatParsers {
    match val {
        TransactionsFormatType::TXT => FormatParsers::Txt,
        TransactionsFormatType::CSV => FormatParsers::Csv,
        TransactionsFormatType::BIN => FormatParsers::Bin,
        _ => panic!("{}", error::ERR_FORMAT),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
           
    pub const TXT_EXAMPLE_IN_PATH: &str = "src\\example\\records_example.txt";
    pub const CSV_EXAMPLE_IN_PATH: &str = "src\\example\\records_example.csv";
    pub const BIN_EXAMPLE_IN_PATH: &str = "src\\example\\records_example.bin";

    pub const TXT_EXAMPLE_OUT_PATH: &str = "src\\example\\test_txt.txt";
    pub const CSV_EXAMPLE_OUT_PATH: &str = "src\\example\\test_csv.csv";
    pub const BIN_EXAMPLE_OUT_PATH: &str = "src\\example\\test_bin.bin";

    #[test]
    fn test_new_tx_create() {
        let tx = Transaction::new();
        assert_eq!(tx.tx_type, TransactionType::EMPTY);
    }

    #[test]
    fn test_format_parsers_create() {
        let parser = get_parser_for_format(TransactionsFormatType::BIN);
        assert_eq!(parser.get_using_format_type(), TransactionsFormatType::BIN);
    }

    #[test]
    fn test_compare_tx_sets() {
        let left_side: Vec<Transaction> = get_example_tx_set();
        let right_side: Vec<Transaction> = get_example_tx_set();
        assert_eq!(compare_tx_sets(left_side, right_side), true);
    }

    #[test]
    fn test_get_format_value() {
        let fmt = String::from("txt");
        assert_eq!(get_format_value(&fmt), TransactionsFormatType::TXT);
    }

    #[test]
    fn test_read_txt()  -> Result<(), Box<dyn std::error::Error>> {

        match File::open(TXT_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let in_parser = get_parser_for_format(TransactionsFormatType::TXT);
                let res_vec = in_parser.from_read(&mut file)?;
                assert_eq!(res_vec.iter().len(), 1000);
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_read_csv() -> Result<(), Box<dyn std::error::Error>> {

        match File::open(CSV_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let in_parser = get_parser_for_format(TransactionsFormatType::CSV);
                let res_vec = in_parser.from_read(&mut file)?;
                assert_eq!(res_vec.iter().len(), 1000);
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_read_bin() -> Result<(), Box<dyn std::error::Error>> {

        match File::open(BIN_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let in_parser = get_parser_for_format(TransactionsFormatType::BIN);
                let res_vec = in_parser.from_read(&mut file)?;
                assert_eq!(res_vec.iter().len(), 1000);
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_write_txt() -> Result<(), Box<dyn std::error::Error>>{

        let test_tx_set: Vec<Transaction> = get_example_tx_set();
        let in_parser = get_parser_for_format(TransactionsFormatType::TXT);
        
        match File::create(Path::new(TXT_EXAMPLE_OUT_PATH)) {
            Ok(mut out_file) => {
                in_parser.write_to(&mut out_file, &test_tx_set)?;
                match File::open(TXT_EXAMPLE_OUT_PATH) {
                    Ok(mut file) => {
                        let res_vec = in_parser.from_read(&mut file)?;
                        assert_eq!(compare_tx_sets(res_vec, get_example_tx_set()), true);
                        std::fs::remove_file(TXT_EXAMPLE_OUT_PATH)?
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_write_csv() -> Result<(), Box<dyn std::error::Error>>{

        let test_tx_set: Vec<Transaction> = get_example_tx_set();
        let in_parser = get_parser_for_format(TransactionsFormatType::CSV);
        
        match File::create(Path::new(CSV_EXAMPLE_OUT_PATH)) {
            Ok(mut out_file) => {
                in_parser.write_to(&mut out_file, &test_tx_set)?;
                match File::open(CSV_EXAMPLE_OUT_PATH) {
                    Ok(mut file) => {
                        let res_vec = in_parser.from_read(&mut file)?;
                        assert_eq!(compare_tx_sets(res_vec, get_example_tx_set()), true);
                        std::fs::remove_file(CSV_EXAMPLE_OUT_PATH)?
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_write_bin() -> Result<(), Box<dyn std::error::Error>>{

        let test_tx_set: Vec<Transaction> = get_example_tx_set();
        let in_parser = get_parser_for_format(TransactionsFormatType::BIN);
        
        match File::create(Path::new(BIN_EXAMPLE_OUT_PATH)) {
            Ok(mut out_file) => {
                in_parser.write_to(&mut out_file, &test_tx_set)?;
                match File::open(BIN_EXAMPLE_OUT_PATH) {
                    Ok(mut file) => {
                        let res_vec = in_parser.from_read(&mut file)?;
                        assert_eq!(compare_tx_sets(res_vec, get_example_tx_set()), true);
                        std::fs::remove_file(BIN_EXAMPLE_OUT_PATH)?
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_txt_to_csv() -> Result<(), Box<dyn std::error::Error>>{

        let txt_parser = get_parser_for_format(TransactionsFormatType::TXT);
        let csv_parser = get_parser_for_format(TransactionsFormatType::CSV);

        match File::open(TXT_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let res_vec = txt_parser.from_read(&mut file)?;
                match File::create(Path::new(CSV_EXAMPLE_OUT_PATH)) {
                    Ok(mut out_file) => {
                        csv_parser.write_to(&mut out_file, &res_vec)?;
                        match File::open(CSV_EXAMPLE_OUT_PATH) {
                            Ok(mut target_file) => {
                                let tareget_vec = csv_parser.from_read(&mut target_file)?;
                                assert_eq!(compare_tx_sets(res_vec, tareget_vec), true);
                                std::fs::remove_file(CSV_EXAMPLE_OUT_PATH)?
                            }
                            Err(e) => {
                                eprintln!("{}", ParserError::Io(e));
                                assert_eq!(false, true)
                            },
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_txt_to_bin() -> Result<(), Box<dyn std::error::Error>>{

        let txt_parser = get_parser_for_format(TransactionsFormatType::TXT);
        let bin_parser = get_parser_for_format(TransactionsFormatType::BIN);

        match File::open(TXT_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let res_vec = txt_parser.from_read(&mut file)?;
                match File::create(Path::new(BIN_EXAMPLE_OUT_PATH)) {
                    Ok(mut out_file) => {
                        bin_parser.write_to(&mut out_file, &res_vec)?;
                        match File::open(BIN_EXAMPLE_OUT_PATH) {
                            Ok(mut target_file) => {
                                let tareget_vec = bin_parser.from_read(&mut target_file)?;
                                assert_eq!(compare_tx_sets(res_vec, tareget_vec), true);
                                std::fs::remove_file(BIN_EXAMPLE_OUT_PATH)?
                            }
                            Err(e) => {
                                eprintln!("{}", ParserError::Io(e));
                                assert_eq!(false, true)
                            },
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_bin_to_csv() -> Result<(), Box<dyn std::error::Error>>{

        let bin_parser = get_parser_for_format(TransactionsFormatType::BIN);
        let csv_parser = get_parser_for_format(TransactionsFormatType::CSV);

        match File::open(BIN_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let res_vec = bin_parser.from_read(&mut file)?;
                match File::create(Path::new(CSV_EXAMPLE_OUT_PATH)) {
                    Ok(mut out_file) => {
                        csv_parser.write_to(&mut out_file, &res_vec)?;
                        match File::open(CSV_EXAMPLE_OUT_PATH) {
                            Ok(mut target_file) => {
                                let tareget_vec = csv_parser.from_read(&mut target_file)?;
                                assert_eq!(compare_tx_sets(res_vec, tareget_vec), true);
                                std::fs::remove_file(CSV_EXAMPLE_OUT_PATH)?
                            }
                            Err(e) => {
                                eprintln!("{}", ParserError::Io(e));
                                assert_eq!(false, true)
                            },
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_bin_to_txt() -> Result<(), Box<dyn std::error::Error>>{

        let bin_parser = get_parser_for_format(TransactionsFormatType::BIN);
        let txt_parser = get_parser_for_format(TransactionsFormatType::TXT);

        match File::open(BIN_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let res_vec = bin_parser.from_read(&mut file)?;
                match File::create(Path::new(TXT_EXAMPLE_OUT_PATH)) {
                    Ok(mut out_file) => {
                        txt_parser.write_to(&mut out_file, &res_vec)?;
                        match File::open(TXT_EXAMPLE_OUT_PATH) {
                            Ok(mut target_file) => {
                                let tareget_vec = txt_parser.from_read(&mut target_file)?;
                                assert_eq!(compare_tx_sets(res_vec, tareget_vec), true);
                                std::fs::remove_file(TXT_EXAMPLE_OUT_PATH)?
                            }
                            Err(e) => {
                                eprintln!("{}", ParserError::Io(e));
                                assert_eq!(false, true)
                            },
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_csv_to_txt() -> Result<(), Box<dyn std::error::Error>>{

        let csv_parser = get_parser_for_format(TransactionsFormatType::CSV);
        let txt_parser = get_parser_for_format(TransactionsFormatType::TXT);

        match File::open(CSV_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let res_vec = csv_parser.from_read(&mut file)?;
                match File::create(Path::new(TXT_EXAMPLE_OUT_PATH)) {
                    Ok(mut out_file) => {
                        txt_parser.write_to(&mut out_file, &res_vec)?;
                        match File::open(TXT_EXAMPLE_OUT_PATH) {
                            Ok(mut target_file) => {
                                let tareget_vec = txt_parser.from_read(&mut target_file)?;
                                assert_eq!(compare_tx_sets(res_vec, tareget_vec), true);
                                std::fs::remove_file(TXT_EXAMPLE_OUT_PATH)?
                            }
                            Err(e) => {
                                eprintln!("{}", ParserError::Io(e));
                                assert_eq!(false, true)
                            },
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    #[test]
    fn test_csv_to_bin() -> Result<(), Box<dyn std::error::Error>>{

        let csv_parser = get_parser_for_format(TransactionsFormatType::CSV);
        let bin_parser = get_parser_for_format(TransactionsFormatType::BIN);

        match File::open(CSV_EXAMPLE_IN_PATH) {
            Ok(mut file) => {
                let res_vec = csv_parser.from_read(&mut file)?;
                match File::create(Path::new(BIN_EXAMPLE_OUT_PATH)) {
                    Ok(mut out_file) => {
                        bin_parser.write_to(&mut out_file, &res_vec)?;
                        match File::open(BIN_EXAMPLE_OUT_PATH) {
                            Ok(mut target_file) => {
                                let target_vec = bin_parser.from_read(&mut target_file)?;
                                assert_eq!(compare_tx_sets(res_vec, target_vec), true);
                                std::fs::remove_file(BIN_EXAMPLE_OUT_PATH)?
                            }
                            Err(e) => {
                                eprintln!("{}", ParserError::Io(e));
                                assert_eq!(false, true)
                            },
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", ParserError::Io(e));
                        assert_eq!(false, true)
                    },
                }
            }
            Err(e) => {
                eprintln!("{}", ParserError::Io(e));
                assert_eq!(false, true)
            },
        }
        Ok(())
    }

    fn get_example_tx_set() -> Vec<Transaction> {

        let tx_1: Transaction = Transaction { 
            tx_id: (1), 
            tx_type: (TransactionType::WITHDRAWAL), 
            from_user_id: (0123456789), 
            to_user_id: (9876543210), 
            amount: (100), 
            timestamp: (0), 
            status: (TransactionStatus::SUCCESS), 
            description: ("Transaction #1".to_owned())
        };

        let tx_2: Transaction = Transaction { 
            tx_id: (1), 
            tx_type: (TransactionType::TRANSFER), 
            from_user_id: (0123456789), 
            to_user_id: (9876543210), 
            amount: (100), 
            timestamp: (0), 
            status: (TransactionStatus::PENDING), 
            description: ("Transaction 2".to_owned())
        };

        let tx_3: Transaction = Transaction { 
            tx_id: (1), 
            tx_type: (TransactionType::DEPOSIT), 
            from_user_id: (0123456789), 
            to_user_id: (9876543210), 
            amount: (100), 
            timestamp: (0), 
            status: (TransactionStatus::FAILURE), 
            description: ("Transaction 3".to_owned())
        };

        vec![
            tx_1, tx_2, tx_3
        ]
    }
}
    