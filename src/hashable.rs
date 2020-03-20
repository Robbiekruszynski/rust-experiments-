pub trait Hashable {
    fn bytes (&self) -> Vec<u8>; //self is like this in JS 
    fn hash (&self) -> Vec<u8> {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}