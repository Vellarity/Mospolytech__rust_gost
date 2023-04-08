
use std::{fs};
use md5;

use rand::{self, Rng};

pub struct Cipher<T> {
    pub mode:String,
    pub bytes_array: Vec<u8>,
    pub encode_struct:T,
    pub password:String,
    pub iv: Vec<u8>
}
/* 
pub trait BasicFunctions {

    fn encode(&self) -> String {
        "123".to_string()
    }
}
 */

impl Cipher<()> {
    pub fn string_to_bytes (input_string:&str) -> Vec<u8> {
        return Vec::from(input_string.as_bytes());
    }
    
    pub fn append_zeros_vec (bytes_arr:&mut Vec<u8>) -> &Vec<u8> {
        while bytes_arr.len() % 16 != 0 {
            bytes_arr.push(0x00)
        }
        return bytes_arr;
    }
    
    pub fn read_file_to_string (file_path:&str) -> String {
        let str_text = fs::read_to_string(file_path).expect("Error: Невозможно прочитать файл по заданному пути");
    
        return str_text;
    } 

    pub fn tokenize_password (password:String) -> String {
        format!("{:x}", md5::compute(password))
    }

    pub fn generate_iv () -> Vec<u8> {
        Vec::from(format!("{:x}", md5::compute(rand::thread_rng().gen::<i64>().to_string())).as_bytes())
    }

    pub fn slice_array_by_cpu (byte_vec:&Vec<u8>) -> Vec<&[u8]> {

        let mut cpus = num_cpus::get();
    
        while cpus / 2 % 2 != 0 {
            cpus += 1
        }
    
        let num_of_blocks = &byte_vec.len() / 16;
    
        let num_of_slices;
        let size_of_slices;
    
        if num_of_blocks < cpus {
            num_of_slices = num_of_blocks;
            size_of_slices = 16
        } else {
            num_of_slices = num_of_blocks / cpus;
            size_of_slices = &byte_vec.len() / num_of_slices; 
        }
    
        println!("{} {}",num_of_blocks, num_of_slices);
    
        let mut result:Vec<&[u8]> = Vec::new();
    
        for i in 0..=num_of_slices-1 {
            let slice: &[u8] = &byte_vec[size_of_slices * i .. size_of_slices * (i+1)];
            result.push(slice);
        }
    
        return result
    }
}

impl Cipher<Vec<u8>> {
    fn encode(&self) -> String {
        let encryption: String;
        match self.mode.as_str() {
            "CBC" => {

            }
            _ => {}
        }

        "123".to_string()
    }
}

impl Cipher<Vec<&[u8]>> {
    fn encode(&self) -> String {
        "123".to_string()
    }
}
