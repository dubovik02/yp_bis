use crate::{
    CVS_HEADER, ParserError, Transaction, TransactionsFormatType, TransactionsParser};

#[derive(Default)]

/// Парсер CSV формата
pub struct CsvParser {
    
}

impl TransactionsParser for CsvParser {
    fn get_using_format_type(&self) -> TransactionsFormatType {
        TransactionsFormatType::CSV
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>, ParserError> {
        let mut result: Vec<Transaction> = Vec::new();

        let mut str_records = String::new();
        source.read_to_string(&mut str_records)?;

        let str_arr: Vec<&str> = str_records
            .split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter(|s| !s.contains("TX_ID"))
            .collect();

        for (numb, stx) in str_arr.iter().enumerate() {
            let tmp_vec: Vec<&str> = stx
            .split(",")
            .map(|s| s.trim())
            .collect();

            if tmp_vec.len() != 8 {
                return Result::Err(ParserError::InvalidCSVStructure(numb));
            }

            let str_tx = 
                "{".to_owned() + 
                "\"tx_id\":" + tmp_vec[0] + (",") + 
                "\"tx_type\":" + "\"" + tmp_vec[1] + "\"" + (",") + 
                "\"from_user_id\":" + tmp_vec[2] + (",") + 
                "\"to_user_id\":" + tmp_vec[3] + (",") + 
                "\"amount\":" + tmp_vec[4] + (",") + 
                "\"timestamp\":" + tmp_vec[5] + (",") + 
                "\"status\":" + "\"" + tmp_vec[6] + "\"" + (",") + 
                "\"description\":" + tmp_vec[7] + 
                "}";
            let tx: Transaction = serde_json::from_str(&str_tx)?;
            result.push(tx);
        }
        Ok(result) 
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError> {
        let mut result_str = String::from(CVS_HEADER);

        for tx in data {
            result_str = result_str.to_owned() + (
                tx.tx_id.to_string() + "," +
                tx.tx_type.to_string().as_str() + "," +
                tx.from_user_id.to_string().as_str() + "," +
                tx.to_user_id.to_string().as_str() + "," +
                tx.amount.to_string().as_str() + "," +
                tx.timestamp.to_string().as_str() + "," +
                tx.status.to_string().as_str() + "," +  
                "\"" + tx.description.to_string().as_str() + "\""
                + "\n"

            ).as_str();
        }
        target.write_all(result_str.as_bytes())?;
        Ok(())
    }
}