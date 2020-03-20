use blockchainlib::*;

fn main() {
    println!("GOOD JOB, YOU IDIOT");
    let block = Block:: new(0, 0, vec![0; 32], 0, "Genesis block".to_owned());
    println!("{:?})", &block);
    let hash = block.hash();
    println!("{:?}", &hash);
}

