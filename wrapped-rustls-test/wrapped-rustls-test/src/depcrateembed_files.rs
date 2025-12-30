// Generated macro for embed_files (macro)
macro_rules! Depcrateembed_files {
() => {
// Module: crate
// Provides: {"embed_files"}
// Dependencies: {}
macro_rules ! embed_files { ($ (($ name : ident , $ keytype : expr , $ path : expr) ;) +) => { $ (const $ name : &'static [u8] = include_bytes ! (concat ! ("../../test-ca/" , $ keytype , "/" , $ path)) ;) + pub fn bytes_for (keytype : & str , path : & str) -> &'static [u8] { match (keytype , path) { $ (($ keytype , $ path) => $ name ,) + _ => panic ! ("unknown keytype {} with path {}" , keytype , path) , } } } }
};
}
