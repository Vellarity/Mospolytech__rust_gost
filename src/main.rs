pub mod algs_func;
pub mod helpers;
pub mod cipher;

//use algs_func::{LSX_encrypt_data,generate_round_keys};

use clap::Parser;
use cipher::Cipher;

//use crate::{helpers::{read_file_to_string, string_to_bytes, append_zeros_vec, slice_array_by_cpu}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    //Режим шифрования: ECB, CBC, CTR
    #[arg(short, long)]
    mode:String,

    //Путь к файлу с текстом, формат txt
    #[arg(short, long)]
    file_path:String,

    //Пароль
    #[arg(short, long)]
    password:String,
}

fn main() {
/*     
    println!("Hello, world!");

    let key: [u8; 32] = [
        0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10,
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef
    ];

    let data: [u8; 16] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x00,
        0xff, 0xee, 0xdd, 0xcc, 0xbb, 0xaa, 0x99, 0x88,
    ];

    let time = time::Instant::now();

    let round_keys = generate_round_keys(key);

    let res = LSX_encrypt_data(round_keys, data);

    println!("{:?}", res.iter().map(|x| format!("{:x}", x)).collect::<Vec<String>>().join(""));

    let res = time.elapsed();

    println!("{:?}", res.as_secs_f32());
 */

    let args = Args::parse();

    let text = Cipher::read_file_to_string(&args.file_path.as_str());
    let mut byte_array = Cipher::string_to_bytes(text.as_str());
    let byte_vec = Cipher::append_zeros_vec(&mut byte_array);

    if args.mode == "CBC" {
        let cipher:Cipher<Vec<u8>> = Cipher {
            mode: args.mode.to_string(),
            bytes_array : byte_vec.to_owned(),
            encode_struct : Vec::new(),
            password: args.password,
            iv: Cipher::generate_iv(),
        };

    }

    if args.mode == "CBC" {

    } else {
        let slices = Cipher::slice_array_by_cpu(&byte_array);


    }

    println!("{:?}", byte_array);
}