// Generated macro for Array (struct)
macro_rules! Depcrate_arrayArray {
() => {
// Module: crate::array
// Provides: {"Array"}
// Dependencies: {}
# [doc = " A TOML [`Value`] that contains a sequence of [`Value`]s"] # [derive (Debug , Default , Clone)] pub struct Array { trailing : RawString , trailing_comma : bool , decor : Decor , pub (crate) span : Option < std :: ops :: Range < usize > > , pub (crate) values : Vec < Item > , }
};
}
