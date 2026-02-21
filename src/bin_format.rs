use crate::{
    BIN_BODY_LEN, MAGIC, 
    Transaction, TransactionStatus, TransactionType, TransactionsFormatType, TransactionsParser,
    error::{ERR_READ_MSG, ERR_WRITE_MSG}
};
use std::io::{BufReader, Read};
use serde_json::Result;

#[derive(Default)]
pub struct BinParser {
    
}

impl TransactionsParser for BinParser {
    fn get_using_format_type(&self) -> TransactionsFormatType {
        TransactionsFormatType::BIN
    }

    fn from_read<R: std::io::Read>(&self, source: &mut R) -> Result<Vec<Transaction>> {

        let mut result: Vec<Transaction> = Vec::new();
        
        let mut reader = BufReader::new(source);

        loop {

            let mut tx = Transaction::new();
            let mut buf4 = [0u8; 4];
            
            reader.read_exact(&mut buf4);

            if String::from(MAGIC) != String::from_utf8_lossy(&buf4).into_owned() {
                break;
            }

            reader.read_exact(&mut buf4).expect(ERR_READ_MSG);

            let mut buf8 = [0u8; 8];
            reader.read_exact(&mut buf8).expect(ERR_READ_MSG);
            tx.tx_id = u64::from_be_bytes(buf8);

            let mut buf1 = [0u8; 1];
            reader.read_exact(&mut buf1).expect(ERR_READ_MSG);
            tx.tx_type = match u8::from_be_bytes(buf1) 
                {
                    0 => TransactionType::DEPOSIT, 
                    1 => TransactionType::TRANSFER, 
                    2 => TransactionType::WITHDRAWAL, 
                    _ => TransactionType::EMPTY
                };

            reader.read_exact(&mut buf8).expect(ERR_READ_MSG);
            tx.from_user_id = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf8).expect(ERR_READ_MSG);
            tx.to_user_id = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf8).expect(ERR_READ_MSG);
            tx.amount = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf8).expect(ERR_READ_MSG);
            tx.timestamp = u64::from_be_bytes(buf8);

            reader.read_exact(&mut buf1).expect(ERR_READ_MSG);
            tx.status = match u8::from_be_bytes(buf1)
            {
                0 => TransactionStatus::SUCCESS,
                1 => TransactionStatus::FAILURE,
                2 => TransactionStatus::PENDING,
                _ => TransactionStatus::EMPTY
            };

            let mut buf_desc_len: [u8; 4] = [0u8; 4];
            reader.read_exact(&mut buf_desc_len).expect(ERR_READ_MSG);

            let desc_len: usize = u32::from_be_bytes(buf_desc_len) as usize;
            let mut buf_desc = vec![0u8; desc_len];
            reader.read_exact(&mut buf_desc).expect(ERR_READ_MSG);
            tx.description = String::from_utf8_lossy(&buf_desc).into_owned();

            result.push(tx);
        }
        Ok(result)
    }

    fn write_to<W: std::io::Write>(&self, target: &mut W, data: Vec<Transaction>) -> Result<()> {

        for tx in data {
            target.write(MAGIC.as_bytes()).expect(ERR_WRITE_MSG);

            let desc_len = tx.description.len();
            let body_len = BIN_BODY_LEN + (desc_len as u32); 
            target.write(&(body_len).to_be_bytes()).expect(ERR_WRITE_MSG);
            
            target.write(&tx.tx_id.to_be_bytes()).expect(ERR_WRITE_MSG);
            target.write(
                match tx.tx_type {
                    TransactionType::DEPOSIT => &[0],
                    TransactionType::TRANSFER => &[1],
                    TransactionType::WITHDRAWAL => &[2],
                    _ => &[3]
                }
            ).expect(ERR_WRITE_MSG);
            target.write(&tx.from_user_id.to_be_bytes()).expect(ERR_WRITE_MSG);
            target.write(&tx.to_user_id.to_be_bytes()).expect(ERR_WRITE_MSG);
            target.write(&tx.amount.to_be_bytes()).expect(ERR_WRITE_MSG);
            target.write(&tx.timestamp.to_be_bytes()).expect(ERR_WRITE_MSG);
            target.write(
                match tx.status {
                    TransactionStatus::SUCCESS => &[0],
                    TransactionStatus::FAILURE => &[1],
                    TransactionStatus::PENDING => &[2],
                    _ => &[3]
                }
            ).expect(ERR_WRITE_MSG);
            
            target.write(&(desc_len as u32).to_be_bytes()).expect(ERR_WRITE_MSG);
            if desc_len != 0 {
                target.write(tx.description.as_bytes()).expect(ERR_WRITE_MSG);
            }
        }
        Ok(())
    }
}