use crate::{
    ParserError, Transaction, TransactionsFormatType, TransactionsParser, txt_to_json_str};

#[derive(Default)]
pub struct TxtParser {
    
}

impl TransactionsParser for TxtParser {
    fn get_using_format_type(&self) -> TransactionsFormatType {
        TransactionsFormatType::TXT
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>, ParserError> {

        let mut result: Vec<Transaction> = Vec::new();

        let mut str_records = String::new();
        source.read_to_string(&mut str_records)?;

        let str_arr: Vec<&str> = str_records
            .split("\n\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();


        for stx in str_arr {

            let tmp_vec: Vec<&str> = stx
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.contains("# Record"))
            .collect();

            let json_vec: Vec<_> = tmp_vec  
            .into_iter()  
            .filter_map(|s| txt_to_json_str(s).ok())
            .collect();

            let str_tx = "{".to_owned() + &json_vec.join(",") + "}";
            let tx: Transaction = serde_json::from_str(&str_tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError> {

        let mut result_str = String::new();

        for (index, tx) in data.iter().enumerate() {
            result_str = result_str.to_owned() + (
                "# Record".to_owned() +  " " 
                    + (index + 1).to_string().as_str() + " " + "(" + tx.tx_type.to_string().as_str() + ")" + "\n" +
                "TX_ID: " + tx.tx_id.to_string().as_str() + "\n" +
                "TX_TYPE: " + tx.tx_type.to_string().as_str() + "\n" +
                "TO_USER_ID: " + tx.to_user_id.to_string().as_str() + "\n" +
                "FROM_USER_ID: " + tx.from_user_id.to_string().as_str() + "\n" +
                "AMOUNT: " + tx.amount.to_string().as_str() + "\n" +
                "TIMESTAMP: " + tx.timestamp.to_string().as_str() + "\n" +
                "STATUS: " + tx.status.to_string().as_str() + "\n" +  
                "DESCRIPTION: " + "\"" + tx.description.to_string().as_str() + "\"" + "\n" +
                "\n"
            ).as_str();
        }
        target.write_all(result_str.as_bytes())?;
        Ok(())
    }
}