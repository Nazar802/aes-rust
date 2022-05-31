use ::phf::{Map, phf_map};

fn input_to_state(input: [u8; 16]) -> [[u8; 4]; 4] {
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    for row in 0..4 { 
        for column in 0..4 {
            state[row][column] = input[row + 4 * column];
        }
    }
    return state;
}

fn state_to_output(state: [[u8; 4]; 4]) -> [u8; 16] {
    let mut output: [u8; 16] = [0; 16];
    for row in 0..4 { 
        for column in 0..4 {
            output[row + 4 * column] = state[row][column];
        }
    }
    return output;
}
 
fn key_expansion() -> [u32; 11] {
    return [0; 11];
}

fn add_round_key (state: [[u8; 4]; 4], round_key: u32) -> [[u8; 4]; 4] {
    return state;
}

fn sub_bytes(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    return state;
}

fn shift_rows(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    return state;
}

fn mix_columns(state: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    return state;
}

fn cipher(input: [u8; 16], rounds: usize) -> [u8; 16] {
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    state = input_to_state(input);
    let mut key_schedule: [u32; 11] = [0; 11];
    key_schedule = key_expansion();

    add_round_key(state, key_schedule[0]);

    for round in 1..rounds-1 {
        sub_bytes(state);
        shift_rows(state);
        mix_columns(state);
        add_round_key(state, key_schedule[round]);
    }

    let output: [u8; 16] = state_to_output(state);
    return output;
}


fn main() {
    const NUMBER_OF_ROUNDS: Map<&str, usize> = phf_map! {
        "4" => 10,
        "6" => 12,
        "8" => 14,
    };

    let input: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let mut output: [u8; 16] = [0; 16];
    let mut state: [[u8; 4]; 4] = [[0; 4]; 4];
    state = input_to_state(input);
    output = state_to_output(state);
    println!("input:\n{:#?}", input);
    println!("state:\n{:#?}", state);
    println!("output:\n{:#?}", output);
    cipher(input, NUMBER_OF_ROUNDS["4"]);
    return;
}
