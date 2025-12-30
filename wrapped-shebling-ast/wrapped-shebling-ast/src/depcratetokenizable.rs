// Generated macro for tokenizable (macro)
macro_rules! Depcratetokenizable {
() => {
// Module: crate
// Provides: {"tokenizable"}
// Dependencies: {}
# [doc = " Utility macro for creating [Token] enums."] macro_rules ! tokenizable { ($ (# [doc = $ enum_doc : expr]) * enum $ name : ident { $ ($ (# [doc = $ arm_doc : expr]) * $ arm : ident ($ token : literal) ,) + }) => { $ (# [doc = $ enum_doc]) * # [derive (Clone , Copy , Debug , PartialEq)] pub enum $ name { $ (# [doc = " ```sh"] # [doc = $ token] # [doc = " ```"] # [doc = ""] $ (# [doc = $ arm_doc]) * $ arm) ,+ } impl self :: Token for $ name { fn token (& self) -> &'static str { match self { $ (Self ::$ arm => $ token) ,+ } } } } ; }
};
}
