use std::{env, fs::File, io, path::Path};

use bis_rust::{Transaction, TransactionsParser, 
    error::ERR_PARAMS, CliParams, get_params, get_parser_for_format};

//pub mod utils;
//use crate::utils::{CliParams, get_params, get_parser_for_format};

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect(); 

    if args.len() < 5 {
        println!("{}", ERR_PARAMS);
        return Ok(());
    }
    
    let params: CliParams = get_params(args, true);

    let in_parser = get_parser_for_format(params.first_file_format);
    let mut file = File::open(params.first_file_name.trim())?;
    let res_vec: Vec<Transaction> = in_parser.from_read(&mut file)?;

    let out_parser = get_parser_for_format(params.second_file_format);

    let mut out_file = File::create(Path::new(params.second_file_name.trim()))?;
    out_parser.write_to(&mut out_file, res_vec)?;

    //////////////////////////////////////////////////////////////////////////////
    println!("Press Enter to exit......");
    let mut key_pressed = String::new();
    io::stdin().read_line(&mut key_pressed)?;
    //////////////////////////////////////////////////////////////////////////////
    Ok(())
}