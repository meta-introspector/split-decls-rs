// Generated macro for impl_120 (impl)
macro_rules! Depcrateimpl_120 {
() => {
// Module: crate
// Provides: {"impl_120"}
// Dependencies: {}
impl Buffer { pub fn new () -> Buffer { Buffer { encoded : Vec :: new () , snappy : Vec :: new () , } } pub fn encode < 'a , T : prost :: Message > (& 'a mut self , message : & T) -> & 'a [u8] { self . encoded . clear () ; message . encode (& mut self . encoded) . expect ("protobuf encoding is infallible") ; self . compress_encoded () } fn compress_encoded (& mut self) -> & [u8] { self . snappy . resize (snap :: raw :: max_compress_len (self . encoded . len ()) , 0) ; let snappy_len = snap :: raw :: Encoder :: new () . compress (& self . encoded , & mut self . snappy) . expect ("snappy encoding is infallible") ; & self . snappy [.. snappy_len] } }
};
}
