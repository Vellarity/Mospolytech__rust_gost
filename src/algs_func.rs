const BLOCK_SIZE:usize = 16;

const L_COEF:[u8; 16] = [
    1, 148, 32, 133, 16, 194, 192, 1,
    251, 1, 192, 194, 16, 133, 32, 148
];

const S_TABLE:[u8; 256] = [
    0xfc, 0xee, 0xdd, 0x11, 0xcf, 0x6e, 0x31, 0x16,
    0xfb, 0xc4, 0xfa, 0xda, 0x23, 0xc5, 0x4, 0x4d,
    0xe9, 0x77, 0xf0, 0xdb, 0x93, 0x2e, 0x99, 0xba,
    0x17, 0x36, 0xf1, 0xbb, 0x14, 0xcd, 0x5f, 0xc1,
    0xf9, 0x18, 0x65, 0x5a, 0xe2, 0x5c, 0xef, 0x21,
    0x81, 0x1c, 0x3c, 0x42, 0x8b, 0x01, 0x8e, 0x4f,
    0x05, 0x84, 0x02, 0xae, 0xe3, 0x6a, 0x8f, 0xa0,
    0x06, 0x0b, 0xed, 0x98, 0x7f, 0xd4, 0xd3, 0x1f,
    0xeb, 0x34, 0x2c, 0x51, 0xea, 0xc8, 0x48, 0xab,
    0xf2, 0x2a, 0x68, 0xa2, 0xfd, 0x3a, 0xce, 0xcc,
    0xb5, 0x70, 0x0e, 0x56, 0x08, 0x0c, 0x76, 0x12,
    0xbf, 0x72, 0x13, 0x47, 0x9c, 0xb7, 0x5d, 0x87,
    0x15, 0xa1, 0x96, 0x29, 0x10, 0x7b, 0x9a, 0xc7,
    0xf3, 0x91, 0x78, 0x6f, 0x9d, 0x9e, 0xb2, 0xb1,
    0x32, 0x75, 0x19, 0x3d, 0xff, 0x35, 0x8a, 0x7e,
    0x6d, 0x54, 0xc6, 0x80, 0xc3, 0xbd, 0x0d, 0x57,
    0xdf, 0xf5, 0x24, 0xa9, 0x3e, 0xa8, 0x43, 0xc9,
    0xd7, 0x79, 0xd6, 0xf6, 0x7c, 0x22, 0xb9, 0x03,
    0xe0, 0x0f, 0xec, 0xde, 0x7a, 0x94, 0xb0, 0xbc,
    0xdc, 0xe8, 0x28, 0x50, 0x4e, 0x33, 0x0a, 0x4a,
    0xa7, 0x97, 0x60, 0x73, 0x1e, 0x00, 0x62, 0x44,
    0x1a, 0xb8, 0x38, 0x82, 0x64, 0x9f, 0x26, 0x41,
    0xad, 0x45, 0x46, 0x92, 0x27, 0x5e, 0x55, 0x2f,
    0x8c, 0xa3, 0xa5, 0x7d, 0x69, 0xd5, 0x95, 0x3b,
    0x07, 0x58, 0xb3, 0x40, 0x86, 0xac, 0x1d, 0xf7,
    0x30, 0x37, 0x6b, 0xe4, 0x88, 0xd9, 0xe7, 0x89,
    0xe1, 0x1b, 0x83, 0x49, 0x4c, 0x3f, 0xf8, 0xfe,
    0x8d, 0x53, 0xaa, 0x90, 0xca, 0xd8, 0x85, 0x61,
    0x20, 0x71, 0x67, 0xa4, 0x2d, 0x2b, 0x09, 0x5b,
    0xcb, 0x9b, 0x25, 0xd0, 0xbe, 0xe5, 0x6c, 0x52,
    0x59, 0xa6, 0x74, 0xd2, 0xe6, 0xf4, 0xb4, 0xc0,
    0xd1, 0x66, 0xaf, 0xc2, 0x39, 0x4b, 0x63, 0xb6
];

fn mul_galoa(mut pol_first:u8, mut pol_second:u8) -> u8 {
    let mut mult_res:u8 = 0;
    let mut hight_bit:u8;

    for _i in 0..8 {
        if (pol_second & 1) != 0 {
            mult_res = mult_res ^ pol_first;
        }

        hight_bit = pol_first & 0x80;

        pol_first = pol_first << 1;

        if hight_bit != 0 {
            pol_first = pol_first ^ 0xc3;
        }

        pol_second = pol_second >> 1;

    }
    return mult_res;
}

fn L_func (data:[u8; 16]) -> u8 {
    let mut la = 0u8;

    for i in 0..BLOCK_SIZE {
        la = la ^ mul_galoa(data[i], L_COEF[i])
    }

    return la
} 

fn L_transform (data:[u8; 16]) -> [u8; 16] {
    let mut la;
    let mut r_data = data;

    for _i in 0..BLOCK_SIZE {
        la = L_func(r_data);

        for j in 0..BLOCK_SIZE-1 {
            r_data[j] = r_data[j+1]
        }

        r_data[BLOCK_SIZE-1] = la
    }

    return r_data;
}

fn S_transform (data:[u8; 16]) -> [u8;16] {
    let mut s_data:[u8; 16] = [0; 16];

    for i in 0 .. BLOCK_SIZE {
        s_data[i] = S_TABLE[data[i] as usize]
    }

    return s_data;
}

fn X_transform (data:[u8; 16], key:[u8;16]) -> [u8;16] {
    let mut x_data = data;

    x_data = x_data.iter().zip(key).map(|(x, y)| x ^ y).collect::<Vec<u8>>().try_into().unwrap();

    return x_data
}

fn data_xor (data_one:[u8;16], data_two:[u8;16]) -> [u8;16] {
    let mut data_res = data_one;

    data_res = data_res.iter().zip(data_two).map(|(x, y)| x ^ y).collect::<Vec<u8>>().try_into().unwrap();

    return data_res;
}

fn LSX_transform (data:[u8; 16], round_key:[u8;16]) -> [u8;16] {
    let mut lsx_data = data;

    lsx_data = X_transform(lsx_data, round_key);
    lsx_data.reverse();
    lsx_data = S_transform(lsx_data);
    lsx_data = L_transform(lsx_data);
    lsx_data.reverse();

    return lsx_data;
}


pub fn init_round_consts () -> [[u8;16];32] {
    let mut round_consts:[[u8;16];32] = [[0;16];32];

    for i in 0 .. round_consts.len() {
        round_consts[i][BLOCK_SIZE-1] = (i+1) as u8;
        round_consts[i].reverse();
        round_consts[i] = L_transform(round_consts[i]);
        round_consts[i].reverse();
    }

    return round_consts;
}

pub fn generate_round_keys (key:[u8;32]) -> [[u8;16];10] {
    let mut round_keys:[[u8;16];10] = [[0;16];10];

    let round_consts = init_round_consts();

    for i in 0..BLOCK_SIZE {
        round_keys[0][i] = key[i];
        round_keys[1][i] = key[i + BLOCK_SIZE];
    }

    let mut k1 = round_keys[0];
    let mut k2 = round_keys[1];
    let mut lsx:[u8; BLOCK_SIZE];

    for i in 0 .. 4 {
        for j in 0 .. 8 {
            if j%2 == 0 {
                lsx = X_transform(k1, round_consts[8*i+j]);
                lsx.reverse();
                lsx = S_transform(lsx);
                lsx = L_transform(lsx);
                lsx.reverse();
                k2 = data_xor(lsx, k2)  
            }
            else if j%2 == 1 {
                lsx = X_transform(k2, round_consts[8 * i + j]);
                lsx.reverse();
                lsx = S_transform(lsx);
                lsx = L_transform(lsx);
                lsx.reverse();
                k1 = data_xor(lsx, k1)
            }
        }
        round_keys[i * 2 + 2] = k1;
        round_keys[i * 2 + 3] = k2; 
    }

    return round_keys;
}

pub fn LSX_encrypt_data (round_keys:[[u8;16];10], data:[u8; 16]) -> [u8; 16] {
    let mut enc_data = data;

    for i in 0 .. 9 {
        enc_data = LSX_transform(enc_data, round_keys[i])
    }
    
    enc_data = X_transform(enc_data, round_keys[9]);

    return enc_data
}