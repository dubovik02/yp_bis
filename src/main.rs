use std::{env, fs::File, path::Path};
use bis_rust::{CliParams, Transaction, TransactionsParser, 
    error::{ERR_PARAMS_CONVERTER, ParserError}, get_params, get_parser_for_format};

fn main() -> Result<(), ParserError> {

    let args: Vec<String> = env::args().collect(); 

    if args.len() < 5 {
        println!("{}", ERR_PARAMS_CONVERTER);
        return Ok(());
    }
    
    let params: CliParams = get_params(args, true);

    let in_parser = get_parser_for_format(params.first_file_format);
    let mut file = File::open(params.first_file_name.trim())?;
    let res_vec: Vec<Transaction> = in_parser.from_read(&mut file)?;

    let out_parser = get_parser_for_format(params.second_file_format);

    let mut out_file = File::create(Path::new(params.second_file_name.trim()))?;
    out_parser.write_to(&mut out_file, &res_vec)?;

    Ok(())
} 