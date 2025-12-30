// Generated macro for impl_150 (impl)
macro_rules! Depcrate_readimpl_150 {
() => {
// Module: crate::read
// Provides: {"impl_150"}
// Dependencies: {}
impl < R : Read > Read for CryptoReader < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { match self { CryptoReader :: Plaintext (r) => r . read (buf) , CryptoReader :: ZipCrypto (r) => r . read (buf) , # [cfg (feature = "aes-crypto")] CryptoReader :: Aes { reader : r , .. } => r . read (buf) , } } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { match self { CryptoReader :: Plaintext (r) => r . read_to_end (buf) , CryptoReader :: ZipCrypto (r) => r . read_to_end (buf) , # [cfg (feature = "aes-crypto")] CryptoReader :: Aes { reader : r , .. } => r . read_to_end (buf) , } } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { match self { CryptoReader :: Plaintext (r) => r . read_to_string (buf) , CryptoReader :: ZipCrypto (r) => r . read_to_string (buf) , # [cfg (feature = "aes-crypto")] CryptoReader :: Aes { reader : r , .. } => r . read_to_string (buf) , } } }
};
}
