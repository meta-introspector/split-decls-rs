// Generated macro for impl_32 (impl)
macro_rules! Depcrate_attrimpl_32 {
() => {
// Module: crate::attr
// Provides: {"impl_32"}
// Dependencies: {}
impl Parser { pub fn new () -> Self { Self { state : Escape :: None , bytes : Vec :: new () , } } fn push (& mut self , c : u8) { self . state = Escape :: None ; self . bytes . push (c) ; } pub fn add (& mut self , c : u8) -> Result < () , Error > { match (self . state , c) { (Escape :: Hex (p) , b'0' ..= b'9') => self . push (p | (c - b'0')) , (Escape :: Hex (p) , b'a' ..= b'f') => self . push (p | (c - b'a' + 10)) , (Escape :: Hex (p) , b'A' ..= b'F') => self . push (p | (c - b'A' + 10)) , (Escape :: Some , b'0' ..= b'9') => self . state = Escape :: Hex ((c - b'0') << 4) , (Escape :: Some , b'a' ..= b'f') => self . state = Escape :: Hex ((c - b'a' + 10) << 4) , (Escape :: Some , b'A' ..= b'F') => self . state = Escape :: Hex ((c - b'A' + 10) << 4) , (Escape :: Some , b' ' | b'"' | b'#' | b'=' | b'\\') => self . push (c) , (Escape :: Some , b'+' | b',' | b';' | b'<' | b'>') => self . push (c) , (Escape :: None , b'\\') => self . state = Escape :: Some , (Escape :: None , ..) => self . push (c) , _ => return Err (ErrorKind :: Failed . into ()) , } Ok (()) } pub fn as_bytes (& self) -> & [u8] { & self . bytes } }
};
}
