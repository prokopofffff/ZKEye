use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let input: u32 = env::read();
    // let s: String = input.to_string();
    // do something with the input
    // writing to the journal makes it public
    // println!("s is {}", &s);
    // let result: u64 = s.as_bytes();
    let numbers: Vec<u32> = (1..=540).collect();
    let sum: u32 = numbers.iter().sum();
    env::commit(&sum);
}