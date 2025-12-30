// Generated macro for UnicodeData (struct)
macro_rules! DepcrateUnicodeData {
() => {
// Module: crate
// Provides: {"UnicodeData"}
// Dependencies: {}
struct UnicodeData { ranges : Vec < (& 'static str , Vec < Range < u32 > >) > , to_upper : BTreeMap < u32 , [u32 ; 3] > , to_lower : BTreeMap < u32 , [u32 ; 3] > , }
};
}
