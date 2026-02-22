use std::{env, fs::File, io};

use bis_rust::{Transaction, TransactionsParser, compare_tx_sets, 
    error::ERR_PARAMS, CliParams, get_params, get_parser_for_format};

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("{}", ERR_PARAMS);
        return Ok(());
    }

    let params: CliParams = get_params(args, false);

    let left_parser = get_parser_for_format(params.first_file_format);

    let mut file_left = File::open(params.first_file_name.trim())?;
    let res_vec_left: Vec<Transaction> = left_parser.from_read(&mut file_left)?;

    let right_parser = get_parser_for_format(params.second_file_format);

    let mut file_right = File::open(params.second_file_name.trim())?;
    let res_vec_right: Vec<Transaction> = right_parser.from_read(&mut file_right)?;

    let result = compare_tx_sets(res_vec_left, res_vec_right);

    if result {
        println!("{}", "Transactions sets are the same.");
    } 
    else {
        println!("{}", "Transactions sets are NOT the same.");
    }

    Ok(())
}