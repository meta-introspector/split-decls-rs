// Generated macro for from_integer (macro)
macro_rules! Depcratefrom_integer {
() => {
// Module: crate
// Provides: {"from_integer"}
// Dependencies: {}
macro_rules ! from_integer { ($ ($ ty : ident) ,*) => { $ (impl From <$ ty > for ConstValue { # [inline] fn from (n : $ ty) -> Self { ConstValue :: Number (n . into ()) } }) * } ; }
};
}
