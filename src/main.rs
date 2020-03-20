use blockchainlib::*;

fn main() {
    println!("RUNNING");
    let block = Block:: new(0, 0, vec![0; 32], 1, "Genesis block".to_owned());
    println!("{:?})", &block);
    let hash = block.hash();
    println!("{:?}", &hash);
   
}

