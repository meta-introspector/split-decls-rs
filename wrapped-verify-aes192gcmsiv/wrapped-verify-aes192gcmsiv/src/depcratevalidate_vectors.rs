// Generated macro for validate_vectors (function)
macro_rules! Depcratevalidate_vectors {
() => {
// Module: crate
// Provides: {"validate_vectors"}
// Dependencies: {}
fn validate_vectors (filename : & Path) { let file = File :: open (filename) . expect ("Failed to open file") ; let reader = io :: BufReader :: new (file) ; let mut vector : Option < VectorArgs > = None ; for line in reader . lines () { let line = line . expect ("Failed to read line") ; let segments : Vec < & str > = line . splitn (2 , " = ") . collect () ; match segments . first () { Some (& "COUNT") => { if let Some (v) = vector . take () { validate (& v) ; } vector = Some (VectorArgs { nonce : String :: new () , key : String :: new () , aad : String :: new () , tag : String :: new () , plaintext : String :: new () , ciphertext : String :: new () , }) ; } Some (& "IV") => { if let Some (v) = & mut vector { v . nonce = segments [1] . parse () . expect ("Failed to parse IV") ; } } Some (& "Key") => { if let Some (v) = & mut vector { v . key = segments [1] . to_string () ; } } Some (& "AAD") => { if let Some (v) = & mut vector { v . aad = segments [1] . to_string () ; } } Some (& "Tag") => { if let Some (v) = & mut vector { v . tag = segments [1] . to_string () ; } } Some (& "Plaintext") => { if let Some (v) = & mut vector { v . plaintext = segments [1] . to_string () ; } } Some (& "Ciphertext") => { if let Some (v) = & mut vector { v . ciphertext = segments [1] . to_string () ; } } _ => { } } } if let Some (v) = vector { validate (& v) ; } }
};
}
