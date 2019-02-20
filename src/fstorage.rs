use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;

//use crc::crc32;

pub struct FileHandler {
    filename_ : String
    //crc_ : u32
}

#[allow(dead_code)]
impl FileHandler {

////////////////////////////////////////////////////////////////////////////////
    pub fn new(filename : &str) -> FileHandler {
    
        FileHandler {
            filename_ : format!("./users/{}.json", filename)
            //crc_ : 0
        }
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn process_and_write_data(&mut self, data : String) -> Result<(), String>{

        let data_as_vec : Vec<u8> = data.into_bytes();

        // CRC
        //self.crc_ = crc32::checksum_ieee(&data_as_vec);
        //let crc_vec = Writer::transform_crc_to_bytes(self.crc_);

        // Append CRC
        //data_as_vec.extend_from_slice(&crc_vec);
        
        // Write Data
        let write_result = self.write(&data_as_vec, true);

        if write_result.is_ok()
        {
            Ok(())
        }
        else
        {
            return Err("Error Writing File".to_string());
        }
    }
                        
////////////////////////////////////////////////////////////////////////////////
    fn write(&self, data : &[u8], create : bool) -> Result<(), std::io::Error> {

        let mut file = OpenOptions::new().
            write(true).
            create(create).
            open(&self.filename_)?;

        file.set_len(0)?;
        file.write(data)?;

        Ok(())
    }

////////////////////////////////////////////////////////////////////////////////
    pub fn read_data(&self) -> Result<String, String> {

        let read_result = FileHandler::read(&self);

        if read_result.is_some()
        {
            return Ok(read_result.unwrap());
        }
        else
        {
            return Err(format!("Error Reading File"));
        }

    }

////////////////////////////////////////////////////////////////////////////////
    fn read(&self) -> Option<String> {
                       
        let file_op = OpenOptions::new().
            read(true).
            create(false).
            open(self.filename_.as_str());

        if file_op.is_err()
        {
            return None
        }
        
        let mut file_reader = std::io::BufReader::new(file_op.unwrap());
        let mut content = String::new();
        if let Ok(_size) = file_reader.read_to_string(&mut content)
        {
            return Some(content);
        }
        else
        {
            return None;
        }
    }
////////////////////////////////////////////////////////////////////////////////
    fn transform_crc_to_bytes(x : u32) -> [u8;4] {
        let b0 : u8 = ((x >> 24) & 0xff) as u8;
        let b1 : u8 = ((x >> 16) & 0xff) as u8;
        let b2 : u8 = ((x >> 8) & 0xff) as u8;
        let b3 : u8 = ((x) & 0xff) as u8;
        
        return [b0, b1, b2, b3];
    }

////////////////////////////////////////////////////////////////////////////////
    fn transform_bytes_to_crc(bytes : &[u8]) -> Result<u32, String> {

        if bytes.len() != 4
        {
            return Err("Byte Array must have length 4".to_string());
        }

        let crc : u32 = (bytes[0] as u32) |
                        (bytes[1] as u32) << 8 |
                        (bytes[2] as u32) << 16 |
                        (bytes[3] as u32) << 24;
        
        Ok(crc)
    }

}
