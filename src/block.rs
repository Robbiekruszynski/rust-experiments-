use std::fmt::{ self, Debug, Formatter };
use super:: *;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    
}

// impl Debug for Block {
//     fn fmt (&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "Block[{}: {} at: {} with {}",
//         &self.index,
//         &self.hash,
//         &self.timestamp,
        
//     )
//     }
// }

impl Block {
    pub fn new ( index: u32, timestamp: u128, prev_block_hash: BlockHash, nonce: u64) -> Self {
            Block {
                index,
                timestamp,
                hash: vec![0; 32],
                prev_block_hash, 
                nonce,
               
            }
        }
}