// Generated macro for impl_218 (impl)
macro_rules! Depcrate_subject_nameimpl_218 {
() => {
// Module: crate::subject_name
// Provides: {"impl_218"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl fmt :: Debug for IpAddrSlice < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . 0 . len () { 4 => { let mut first = true ; for byte in self . 0 { match first { true => first = false , false => f . write_str (".") ? , } write ! (f , "{byte}") ? ; } Ok (()) } 16 => { let (mut first , mut skipping) = (true , false) ; for group in self . 0 . chunks_exact (2) { match (first , group == [0 , 0] , skipping) { (true , _ , _) => first = false , (false , false , false) => f . write_str (":") ? , (false , true , _) => { skipping = true ; continue ; } (false , false , true) => { skipping = false ; f . write_str ("::") ? ; } } if group [0] != 0 { write ! (f , "{:x}" , group [0]) ? ; } match group [0] { 0 => write ! (f , "{:x}" , group [1]) ? , _ => write ! (f , "{:02x}" , group [1]) ? , } } Ok (()) } _ => { f . write_str ("[invalid: ") ? ; let mut first = true ; for byte in self . 0 { match first { true => first = false , false => f . write_str (", ") ? , } write ! (f , "{byte:02x}") ? ; } f . write_str ("]") } } } }
};
}
