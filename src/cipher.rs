use std::{fs, time, process::exit};
use md5;

use rand::{self, Rng};

use crate::algs_func;

use kuznechik::{KeyStore, Kuznechik, AlgEcb, AlgCbc, AlgCtr};
use aes::Aes128;
use aes::cipher::{ BlockEncrypt, KeyInit, KeyIvInit, generic_array::GenericArray, BlockEncryptMut, StreamCipherCoreWrapper, StreamCipher};
type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;
type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;


pub trait Encoder {
    fn encode(&self) { println!("Empty implementation for all types") }
}

pub struct BasicFunctions;

impl BasicFunctions {
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

    pub fn tokenize_password (password:String) -> [u8; 16] {
        let md_hash = format!("{:?}", md5::compute(&password));

        let mut result:[u8; 16] = [0u8; 16];

        hex::decode_to_slice(md_hash, &mut result).expect("Ошибка в просчёте хэша");

        return result
    }

    pub fn generate_iv () -> Vec<u8> {
        Vec::from(format!("{:x}", md5::compute(rand::thread_rng().gen::<i128>().to_string())).as_bytes())
    }

}

pub struct GostCipher {
    pub mode:String,
    pub text: String,
    pub pass: String,
    pub bytes_array: Vec<u8>,
    pub encode_struct:Vec<u8>,
    pub password: [u8; 16],
    pub iv: Vec<u8>,
}

impl Encoder for GostCipher {
    fn encode(&self) {
        let mut encryption_massive = vec![];
        
        let key:[u8; 32] = self.password.map(|i| format!("{:02x}", i)).join("").as_bytes().try_into().expect("Неверное количество символов");
        
        let round_keys = algs_func::generate_round_keys(key);
        match self.mode.as_str() {
            "ECB" => {
                let real_timer = time::Instant::now();
                unsafe {
                    let clock_timer = core::arch::x86_64::_rdtsc();
                    for i in 0..(self.encode_struct.len() / 16) {
                        let data_to_encode:[u8;16] = self.encode_struct[16*i .. 16*(i+1)].to_owned().try_into().unwrap();
                        let res = algs_func::LSX_encrypt_data(round_keys, data_to_encode);
                        encryption_massive.push(res);
                    }
                    println!(
                        "IV: {:} \nSize of message: {:} bytes \nElapsed real time: {:} ns \nElapsed clocks: {:} \n", 
                        std::str::from_utf8(&self.iv[..]).unwrap(),
                        self.encode_struct.len(),
                        real_timer.elapsed().as_nanos(), 
                        core::arch::x86_64::_rdtsc() - clock_timer
                    );

                    //println!("Encrypted message: {} \n", encryption_massive.into_iter().flatten().collect::<Vec<u8>>().iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(""),)
                }; 
            }
            "ECB_LIB" => {
                let kuz = KeyStore::new().password(&self.pass);

                let data = Vec::from(self.text.as_str());

                let real_timer = time::Instant::now();
                unsafe {
                    let clock_timer = core::arch::x86_64::_rdtsc();
                    let cipher = AlgEcb::new(&kuz).encrypt(data);

                    println!(
                        "Size of message: {:} bytes\n Elapsed real time: {:} ns\n Elapsed clocks: {:} \n", 
                        self.text.len(),
                        real_timer.elapsed().as_nanos(), 
                        core::arch::x86_64::_rdtsc() - clock_timer
                    );
                    //println!("Encrypted message: {:#100} \n ", cipher.iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(""))
                }
                
            }
            "CBC_LIB" => {
                let kuz = KeyStore::new().password(&self.pass);

                let gamma = self.iv.clone();

                let data = Vec::from(self.text.as_str());

                let real_timer = time::Instant::now();
                unsafe {
                    let clock_timer = core::arch::x86_64::_rdtsc();
                    let cipher = AlgCbc::new(&kuz).gamma(gamma.clone()).encrypt(data);

                    println!(
                        "Size of message: {:} bytes\n Elapsed real time: {:} ns\n Elapsed clocks: {:} \n", 
                        self.text.len(),
                        real_timer.elapsed().as_nanos(), 
                        core::arch::x86_64::_rdtsc() - clock_timer
                    );

                    println!("Encrypted message: {:#100} \nIV: {} \n", cipher.iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(""), std::str::from_utf8(&self.iv[..]).unwrap(),)
                }
            }
            "CTR_LIB" => {
                let kuz = KeyStore::new().password(&self.pass);

                let gamma = self.iv.clone();

                let data = Vec::from(self.text.as_str());

                let real_timer = time::Instant::now();
                unsafe {
                    let clock_timer = core::arch::x86_64::_rdtsc();
                    let cipher = AlgCtr::new(&kuz).gamma(gamma.clone()).encrypt(data);

                    println!(
                        "Size of message: {:} bytes\n Elapsed real time: {:} ns\n Elapsed clocks: {:} \n", 
                        self.text.len(),
                        real_timer.elapsed().as_nanos(), 
                        core::arch::x86_64::_rdtsc() - clock_timer
                    );

                    println!("Encrypted message: {:#100} \nIV: {} \n", cipher.iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(""), std::str::from_utf8(&self.iv[..]).unwrap(),)
                }
            }
            _ => {
                println!("Данный режим не поддерживается.");
                exit(2)
            }
        }
    }
}

pub struct AesCipher {
    pub mode:String,
    pub text: String,
    pub pass: String,
    pub bytes_array: Vec<u8>,
    pub encode_struct:Vec<u8>,
    pub password: [u8; 16],
    pub iv: Vec<u8>,
}

impl Encoder for AesCipher {
    fn encode(&self) {
        let mut encryption_massive = vec![];

        match self.mode.as_str() {
            "ECB_LIB" => {
                let key = GenericArray::from(self.password);
                let cipher = Aes128::new(&key);
                let real_timer = time::Instant::now();
                unsafe {
                    let clock_timer = core::arch::x86_64::_rdtsc();
                    for i in 0..(self.encode_struct.len() / 16) {
                        let data_to_gen:[u8;16] = self.encode_struct[16*i .. 16*(i+1)].to_owned().try_into().unwrap();
                        let mut data_to_encode = GenericArray::from(data_to_gen);
                        cipher.encrypt_block(&mut data_to_encode);
                        encryption_massive.push(data_to_encode);
                    }
                    println!(
                        "IV: {:} \nSize of message: {:} bytes \nElapsed real time: {:} ns \nElapsed clocks: {:} \n", 
                        std::str::from_utf8(&self.iv[..]).unwrap(),
                        self.encode_struct.len(),
                        real_timer.elapsed().as_nanos(), 
                        core::arch::x86_64::_rdtsc() - clock_timer
                    );
                    //println!("Encrypted message: {} \n", encryption_massive.into_iter().flatten().collect::<Vec<u8>>().iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(""),)
                }; 
            }
            "CBC_LIB" => {
                let key = GenericArray::from(self.password);
                let iv = [0x24; 16];
                let mut cipher = Aes128CbcEnc::new(&key, &iv.into());
                let real_timer = time::Instant::now();
                unsafe {
                    let clock_timer = core::arch::x86_64::_rdtsc();
                    for i in 0..(self.encode_struct.len() / 16) {
                        let data_to_gen:[u8;16] = self.encode_struct[16*i .. 16*(i+1)].to_owned().try_into().unwrap();
                        let mut data_to_encode = GenericArray::from(data_to_gen);
                        cipher.encrypt_block_mut(&mut data_to_encode);
                    
                        encryption_massive.push(data_to_encode);
                    }
                    println!(
                        "IV: {:} \nSize of message: {:} bytes \nElapsed real time: {:} ns \nElapsed clocks: {:} \n", 
                        std::str::from_utf8(&self.iv[..]).unwrap(),
                        self.encode_struct.len(),
                        real_timer.elapsed().as_nanos(), 
                        core::arch::x86_64::_rdtsc() - clock_timer
                    );
                }
            }
            "CTR_LIB" => {
                let key = GenericArray::from(self.password);
                let iv = [0x24; 16];
                let mut cipher = Aes128Ctr64LE::new(&key, &iv.into());
                let mut text = self.encode_struct.clone();
                let real_timer = time::Instant::now();
                unsafe {
                    let clock_timer = core::arch::x86_64::_rdtsc();
                    cipher.apply_keystream(&mut text);
                    //encryption_massive.push(GenericArray::from(text.as_slice()));
    
                    println!(
                        "IV: {:} \nSize of message: {:} bytes \nElapsed real time: {:} ns \nElapsed clocks: {:} \n", 
                        std::str::from_utf8(&self.iv[..]).unwrap(),
                        self.encode_struct.len(),
                        real_timer.elapsed().as_nanos(), 
                        core::arch::x86_64::_rdtsc() - clock_timer
                    );
                    //println!("Encrypted message: {} \n", encryption_massive.into_iter().flatten().collect::<Vec<u8>>().iter().map(|i| format!("{:x}", i)).collect::<Vec<String>>().join(""),)
                }; 
            }
            _ => {
                println!("Данный режим не поддерживается.");
                exit(2)
            }
        }
    }
}