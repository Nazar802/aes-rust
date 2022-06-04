use ::phf::{Map, phf_map};

/*
Constants
*/

const NUMBER_OF_ROUNDS: Map<&str, usize> = phf_map! {
    "4" => 10,
    "6" => 12,
    "8" => 14,
};

const KEY_SCHEDULE_LENGTH: Map<&str, usize> = phf_map! {
    "4" => 44,
    "6" => 66,
    "8" => 88,
};

const S_BOX: [u8; 256] = [
//   0     1     2     3     4     5     6     7     8     9     A     B     C     D    E      F   
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76, // 0
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0, // 1
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15, // 2
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75, // 3
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84, // 4
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF, // 5
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8, // 6
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2, // 7
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73, // 8
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB, // 9
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79, // A
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08, // B
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A, // C
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E, // D
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF, // E
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16, // F
];

const INV_S_BOX: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D,
];

const RCON: [[u8; 4]; 10] = [
    [0x01, 0x00, 0x00, 0x00],
    [0x02, 0x00, 0x00, 0x00],
    [0x04, 0x00, 0x00, 0x00],
    [0x08, 0x00, 0x00, 0x00],
    [0x10, 0x00, 0x00, 0x00],
    [0x20, 0x00, 0x00, 0x00],
    [0x40, 0x00, 0x00, 0x00],
    [0x80, 0x00, 0x00, 0x00],
    [0x1B, 0x00, 0x00, 0x00],
    [0x36, 0x00, 0x00, 0x00], 
];

/*
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] -> [[0,1,2,3]...]

*/
fn input_to_state(input: [u8; 16]) -> [[u8; 4]; 4] {
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    for row in 0..4 { 
        for column in 0..4 {
            state[row][column] = input[4 * row + column];
        }
    }
    return state;
}

fn state_to_output(state: [[u8; 4]; 4]) -> [u8; 16] {
    let mut output: [u8; 16] = [0; 16];
    for row in 0..4 { 
        for column in 0..4 {
            output[4 * row + column] = state[row][column];
        }
    }
    return output;
}

fn invert_state(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut transposed: [[u8; 4]; 4] = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            transposed[j][i] = state[i][j];
        }
    }
    return transposed;
}

fn xor(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    let mut byte_xor: [u8; 4] = [0; 4];
    for i in 0..4 {
        byte_xor[i] = a[i] ^ b[i];
    }
    return byte_xor;
}

/*
KEY EXPANSION
*/

fn sub_word(mut word: [u8; 4]) -> [u8; 4] {
    for i in 0..4 {
        //println!("\nInitital\n{:#X}", word[i]);
        word[i] = S_BOX[word[i] as usize];
        //println!("S_BOXED\n{:#X}", word[i]);
    }
    return word;
}

fn rot_word(mut word: [u8; 4]) -> [u8; 4] {
    let mut saved_byte: u8 = 0;
    saved_byte = word[0];
    for i in 1..4 {
        word[i-1] = word[i];
    }
    word[3] = saved_byte;
    return word;
}

fn key_expansion(key: [u8; 16], key_length: usize) -> [[[u8; 4]; 4]; 11] {
    let mut key_arr: [[u8; 4]; 4] = [[0; 4]; 4];
    key_arr = input_to_state(key);
    //println!("key array:\n{:#?}", key_arr);
    let mut init_key_schedule: [[u8; 4]; 44] = [[0; 4]; 44]; 
    let mut key_schedule:[[[u8; 4]; 4]; 11] = [[[0; 4]; 4]; 11];
    for i in 0..4 {
        init_key_schedule[i] = key_arr[i];
    }
    
    let mut temp: [u8; 4];
    for i in key_length.. 4 * (NUMBER_OF_ROUNDS["4"] + 1) {
        temp = init_key_schedule[i-1];
        
        if i % key_length == 0 {
            temp = xor(sub_word(rot_word(temp)), RCON[(i / key_length) - 1]);
        }
        else if key_length > 6 && i % key_length == 4 {
            temp = sub_word(temp);
        }
        
        init_key_schedule[i] = xor(init_key_schedule[i-key_length], temp);
    }
    //println!("44x4 key schedule:\n{:#X?}", init_key_schedule);
    for i in 0..NUMBER_OF_ROUNDS["4"] + 1 {
        for j in 0..4 {
            for k in 0..4 {
                key_schedule[i][j][k] = init_key_schedule[4 * i + j][k];
            }
        }
    }

    return key_schedule;
}


fn add_round_key (mut state: [[u8; 4]; 4], round_key: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] ^= round_key[i][j];
        }
    }
    return state;
}

fn sub_bytes(mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = S_BOX[state[i][j] as usize];
        }
    }
    return state;
}

fn inv_sub_bytes(mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = INV_S_BOX[state[i][j] as usize];
        }
    }
    return state;
}

fn shift_rows(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut permutated: [[u8; 4]; 4] = [[0; 4]; 4];
    permutated = state;
    permutated[0][1] = state[1][1];
    permutated[1][1] = state[2][1];
    permutated[2][1] = state[3][1];
    permutated[3][1] = state[0][1];
    permutated[0][2] = state[2][2];
    permutated[1][2] = state[3][2];
    permutated[2][2] = state[0][2];
    permutated[3][2] = state[1][2];
    permutated[0][3] = state[3][3];
    permutated[1][3] = state[0][3];
    permutated[2][3] = state[1][3];
    permutated[3][3] = state[2][3];
    return permutated;
}

fn inv_shift_rows(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut permutated: [[u8; 4]; 4] = [[0; 4]; 4];
    permutated = state;
    permutated[0][1] = state[3][1];
    permutated[1][1] = state[0][1];
    permutated[2][1] = state[1][1];
    permutated[3][1] = state[2][1];
    permutated[0][2] = state[2][2];
    permutated[1][2] = state[3][2];
    permutated[2][2] = state[0][2];
    permutated[3][2] = state[1][2];
    permutated[0][3] = state[1][3];
    permutated[1][3] = state[2][3];
    permutated[2][3] = state[3][3];
    permutated[3][3] = state[0][3];
    return permutated;
}

//xtime = lambda a: (((a << 1) ^ 0x1B) & 0xFF) if (a & 0x80) else (a << 1);
fn xtime(x: u8) -> u8 {
    let mut result: u8 = 0;
    if x & 0x80 != 0 {
        result = ((x << 1) ^ 0x1B) & 0xFF;
    }
    else {
        result = x << 1;
    }
    return result;
}

fn mix_single_column(mut column: [u8; 4]) -> [u8; 4] {
    let mut t: u8 = 0;
    let mut u: u8 = 0;
    t = column[0] ^ column[1] ^ column[2] ^ column[3];
    u = column[0];
    column[0] ^= t ^ xtime(column[0] ^ column[1]);
    column[1] ^= t ^ xtime(column[1] ^ column[2]);
    column[2] ^= t ^ xtime(column[2] ^ column[3]);
    column[3] ^= t ^ xtime(column[3] ^ u);
    return column;
}

fn mix_columns(mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    for i in 0..4 {
        state[i] = mix_single_column(state[i]);
    }   
    return state;
}

fn inv_mix_columns(mut state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut u: u8 = 0;
    let mut v: u8 = 0;
    for i in 0..4 {
        u = xtime(xtime(state[i][0] ^ state[i][2]));
        v = xtime(xtime(state[i][1] ^ state[i][3]));
        state[i][0] ^= u;
        state[i][1] ^= v;
        state[i][2] ^= u;
        state[i][3] ^= v;
    }
    state = mix_columns(state);
    return state;
}

fn cipher_block(input: [u8; 16], key: [u8; 16], rounds: usize) -> [u8; 16] {
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    state = input_to_state(input);
    let mut key_schedule: [[[u8; 4]; 4]; 11] = [[[0; 4]; 4]; 11];
    key_schedule = key_expansion(key, 4);

    state = add_round_key(state, key_schedule[0]);
    
    //println!("round: 0");
    //println!("state:\n{:#X?}", state);

    for round in 1..rounds {
        state = sub_bytes(state);
        state = shift_rows(state);
        state = mix_columns(state);
        state = add_round_key(state,key_schedule[round]);
        //println!("round:{:#?}", round);
        //println!("state:\n{:#X?}", state);
    }
    state = sub_bytes(state);
    state = shift_rows(state);
    state = add_round_key(state,key_schedule[rounds]);

    //println!("round: 10");
    //println!("state:\n{:#X?}", state);

    let output: [u8; 16] = state_to_output(state);
    return output;
}

fn inv_cipher_block(input: [u8; 16], key: [u8; 16], rounds: usize) -> [u8; 16] {
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    state = input_to_state(input);
    let mut key_schedule: [[[u8; 4]; 4]; 11] = [[[0; 4]; 4]; 11];
    key_schedule = key_expansion(key, 4);

    state = add_round_key(state, key_schedule[10]);

    println!("round: 0");
    println!("state:\n{:#X?}", state);
    for round in 1..rounds {
        state = inv_shift_rows(state);
        state = inv_sub_bytes(state);
        state = add_round_key(state,key_schedule[rounds - round]);
        state = inv_mix_columns(state);
        println!("round:{:#?}", round);
        println!("state:\n{:#X?}", state);
    }

    state = inv_shift_rows(state);
    state = inv_sub_bytes(state);
    state = add_round_key(state,key_schedule[0]);
    println!("round: 10");
    println!("state:\n{:#X?}", state);
    let output: [u8; 16] = state_to_output(state);
    return output;
}


fn main() {
    
    let input: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    //let mut output: [u8; 16] = [0; 16];
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    state = input_to_state(input);
    //output = state_to_output(state);
    println!("input:\n{:#?}", input);
    println!("state:\n{:#?}", state);
    let mut output: [u8; 16] = [0; 16];

    let mut key: [u8; 16] = [0; 16];
    //let mut key_schedule:[[[u8; 4]; 4]; 11] = [[[0; 4]; 4]; 11];

    //key_schedule = key_expansion(key, 4);

    //println!("key schedule:\n{:#X?}", key_schedule);

    output = cipher_block(input, key, 10);
    
    println!("Encrypted:\n{:#X?}", output);
    
    output = inv_cipher_block(output, key, 10);
    println!("Decrypted:\n{:#X?}", output);

    //let a: [u8; 4] = [1, 2, 3, 4];
    //let b: [u8; 4] = [4, 3, 2, 1];
    

    //let mut key: [u8; 16] = [0x1d, 0xa1, 0xcf, 0xc8, 0x0c, 0xa4, 0xe4, 0xba, 0x9f, 0x33, 0x25, 0x55, 0xcf, 0xfa, 0x1d, 0xc2];
    
    /*
    state = sub_bytes(state);
    println!("SubBytes:\n{:#X?}", state);

    state = shift_rows(state);
    println!("ShiftRows:\n{:#X?}", state);

    state = mix_columns(state);
    println!("MixColumns:\n{:#X?}", state);

    state = add_round_key(state, key_schedule[1]);
    println!("Added key to state:\n{:#X?}", state);
    */
    /*
    let mut xor_arr: [u8; 4] = [0; 4];

    xor_arr = xor(a, b);
    println!("XOR:\n{:#?}", xor_arr);
    println!("output:\n{:#?}", output);
    */
    //cipher(input, NUMBER_OF_ROUNDS["4"]);
    
    /*
    let mut word: [u8; 4] = [0x1d, 0xa1, 0xcf, 0xc8];
    word = rot_word(word);
    println!("word:\n{:#X?}", word);
    */
    return;
}
