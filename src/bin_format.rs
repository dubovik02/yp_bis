use crate::{
    BIN_BODY_LEN, MAGIC, ParserError, Transaction, TransactionStatus, TransactionType, TransactionsFormatType, TransactionsParser};
use std::io::{self, BufReader, Read};

#[derive(Default)]
pub struct BinParser {
    
}

impl TransactionsParser for BinParser {
    fn get_using_format_type(&self) -> TransactionsFormatType {
        TransactionsFormatType::BIN
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>, ParserError> {

        let mut result: Vec<Transaction> = Vec::new();
        
        let mut reader = BufReader::new(source);

        loop {

            let mut tx = Transaction::new();
            let mut buf4 = [0u8; 4];

            match reader.read_exact(&mut buf4) {
                Ok(_) => (),
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => {
                    return Err(ParserError::Io(e))
                },
            }

            if String::from(MAGIC) != String::from_utf8_lossy(&buf4).into_owned() {
                break;
            }

            reader.read_exact(&mut buf4)?;

            let mut buf8 = [0u8; 8];
            reader.read_exact(&mut buf8)?;
            tx.tx_id = u64::from_be_bytes(buf8);

            let mut buf1 = [0u8; 1];
            reader.read_exact(&mut buf1)?;
            tx.tx_type = match u8::from_be_bytes(buf1) 
                {
                    0 => TransactionType::DEPOSIT, 
                    1 => TransactionType::TRANSFER, 
                    2 => TransactionType::WITHDRAWAL, 
                    _ => TransactionType::EMPTY
                };

            reader.read_exact(&mut buf8)?;
            tx.from_user_id = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf8)?;
            tx.to_user_id = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf8)?;
            tx.amount = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf8)?;
            tx.timestamp = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf1)?;
            tx.status = match u8::from_be_bytes(buf1)
            {
                0 => TransactionStatus::SUCCESS,
                1 => TransactionStatus::FAILURE,
                2 => TransactionStatus::PENDING,
                _ => TransactionStatus::EMPTY
            };

            let mut buf_desc_len: [u8; 4] = [0u8; 4];
            reader.read_exact(&mut buf_desc_len)?;

            let desc_len: usize = u32::from_be_bytes(buf_desc_len) as usize;
            let mut buf_desc = vec![0u8; desc_len];
            reader.read_exact(&mut buf_desc)?;
            tx.description = String::from_utf8_lossy(&buf_desc).into_owned().replace("\"", "");
            result.push(tx);
        }
        Ok(result)
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: &Vec<Transaction>) -> Result<(), ParserError> {

        for tx in data {
            target.write(MAGIC.as_bytes())?;

            let desc_len = tx.description.len();
            let body_len = BIN_BODY_LEN + (desc_len as u32); 
            target.write(&(body_len).to_be_bytes())?;
            
            target.write(&tx.tx_id.to_be_bytes())?;
            target.write(
                match tx.tx_type {
                    TransactionType::DEPOSIT => &[0],
                    TransactionType::TRANSFER => &[1],
                    TransactionType::WITHDRAWAL => &[2],
                    _ => &[3]
                }
            )?;
            target.write(&tx.from_user_id.to_be_bytes())?;
            target.write(&tx.to_user_id.to_be_bytes())?;
            target.write(&tx.amount.to_be_bytes())?;
            target.write(&tx.timestamp.to_be_bytes())?;
            target.write(
                match tx.status {
                    TransactionStatus::SUCCESS => &[0],
                    TransactionStatus::FAILURE => &[1],
                    TransactionStatus::PENDING => &[2],
                    _ => &[3]
                }
            )?;
            
            target.write(&((desc_len + 2) as u32).to_be_bytes())?;
            if desc_len != 0 {
                let desc_str = "\"".to_owned() + tx.description.as_str() + "\"";
                target.write(desc_str.as_bytes())?;
            }
        }
        Ok(())
    }
}