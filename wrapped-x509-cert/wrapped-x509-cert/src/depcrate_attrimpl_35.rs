// Generated macro for impl_35 (impl)
macro_rules! Depcrate_attrimpl_35 {
() => {
// Module: crate::attr
// Provides: {"impl_35"}
// Dependencies: {}
# [doc = " Serializes the structure according to the rules in [RFC 4514]."] # [doc = ""] # [doc = " [RFC 4514]: https://datatracker.ietf.org/doc/html/rfc4514"] impl fmt :: Display for AttributeTypeAndValue { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let val = match self . value . tag () { Tag :: PrintableString => PrintableStringRef :: try_from (& self . value) . ok () . map (| s | s . as_str ()) , Tag :: Utf8String => Utf8StringRef :: try_from (& self . value) . ok () . map (| s | s . as_str ()) , Tag :: Ia5String => Ia5StringRef :: try_from (& self . value) . ok () . map (| s | s . as_str ()) , Tag :: TeletexString => TeletexStringRef :: try_from (& self . value) . ok () . map (| s | s . as_str ()) , _ => None , } ; if let (Some (key) , Some (val)) = (DB . shortest_name_by_oid (& self . oid) , val) { write ! (f , "{}=" , key . to_ascii_uppercase ()) ? ; let mut iter = val . char_indices () . peekable () ; while let Some ((i , c)) = iter . next () { match c { '#' if i == 0 => write ! (f , "\\#") ? , ' ' if i == 0 || iter . peek () . is_none () => write ! (f , "\\ ") ? , '"' | '+' | ',' | ';' | '<' | '>' | '\\' => write ! (f , "\\{c}") ? , '\x00' ..= '\x1f' | '\x7f' => write ! (f , "\\{:02x}" , c as u8) ? , _ => f . write_char (c) ? , } } } else { let value = self . value . to_der () . or (Err (fmt :: Error)) ? ; write ! (f , "{}=#" , self . oid) ? ; for c in value { write ! (f , "{c:02x}") ? ; } } Ok (()) } }
};
}
