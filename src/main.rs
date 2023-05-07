pub mod algs_func;
pub mod helpers;
pub mod cipher;

//use algs_func::{LSX_encrypt_data,generate_round_keys};

use std::process::exit;

use clap::Parser;
use cipher::{GostCipher, Encoder, BasicFunctions, AesCipher};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Алгоритм: AES, GOST
    #[arg(short, long)]
    algorithm:String,

    /// Режим шифрования: ECB, ECB_LIB, CBC_LIB, CTR_LIB
    #[arg(short, long)]
    mode:String,

    /// Путь к файлу с текстом, формат txt
    #[arg(short, long)]
    file_path:String,

    /// Пароль
    #[arg(short, long)]
    password:String,
}

fn main() {
    let args = Args::parse();

    let text = BasicFunctions::read_file_to_string(&args.file_path.as_str());
    let mut byte_array = BasicFunctions::string_to_bytes(&text.as_str());
    let byte_vec = BasicFunctions::append_zeros_vec(&mut byte_array);

    match args.algorithm.as_str() {
        "GOST" => {
            let cipher = GostCipher {
                text: text,
                pass: args.password.clone(),
                mode: args.mode.to_owned(),
                bytes_array : byte_vec.to_owned(),
                encode_struct : byte_vec.to_owned(),
                password: BasicFunctions::tokenize_password(args.password),
                iv: BasicFunctions::generate_iv(),
            };
            
            cipher.encode();            
        }
        "AES" => {
            let cipher = AesCipher {
                text: text,
                pass: args.password.clone(),
                mode: args.mode.to_owned(),
                bytes_array : byte_vec.to_owned(),
                encode_struct : byte_vec.to_owned(),
                password: BasicFunctions::tokenize_password(args.password),
                iv: BasicFunctions::generate_iv(),
            };

            cipher.encode();            
        }
        _ => {
            println!("Данный алгоритм не поддерживается");
            exit(1)
        }
    }
}