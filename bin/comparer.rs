use std::{env, fs::File, io};

use bis_rust::{Transaction, TransactionsParser, compare_tx_sets, 
    error::ERR_PARAMS, CliParams, get_params, get_parser_for_format};

// ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
// # Output: The transaction records in 'records_example.bin' and 'records_example.csv' are identical.

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("{}", ERR_PARAMS);
        return Ok(());
    }

    let params: CliParams = get_params(args, false);

    //////////////////////////////////////////////////////////////////////////////
    println!("-------------------------------------");
    println!("File #1 : {}", params.first_file_name.trim());
    println!("File #2 : {}", params.second_file_name.trim());
    println!("-------------------------------------");
    //////////////////////////////////////////////////////////////////////////////

    let left_parser = get_parser_for_format(params.first_file_format);

    let mut file_left = File::open(params.first_file_name.trim())?;
    let res_vec_left: Vec<Transaction> = left_parser.from_read(&mut file_left)?;

    let right_parser = get_parser_for_format(params.second_file_format);

    let mut file_right = File::open(params.second_file_name.trim())?;
    let res_vec_right: Vec<Transaction> = right_parser.from_read(&mut file_right)?;

    //////////////////////////////////////////////////////////////////////////////
    println!("-------------------------------------");
    println!("File #1 has {} items", res_vec_left.len());
    println!("File #2 has {} items", res_vec_right.len());
    println!("-------------------------------------");
    //////////////////////////////////////////////////////////////////////////////

    let result = compare_tx_sets(res_vec_left, res_vec_right);

    if result {
        println!("{}", "Transactions sets are the same.");
    } 
    else {
        println!("{}", "Transactions sets are NOT the same.");
    }
    println!("-------------------------------------");
    //////////////////////////////////////////////////////////////////////////////
    println!("Press Enter to exit......");
    let mut key_pressed = String::new();
    io::stdin().read_line(&mut key_pressed)?;
    //////////////////////////////////////////////////////////////////////////////
    Ok(())
}