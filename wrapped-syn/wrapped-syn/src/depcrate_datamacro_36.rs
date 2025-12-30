// Generated macro for macro_36 (macro)
macro_rules! Depcrate_datamacro_36 {
() => {
// Module: crate::data
// Provides: {"macro_36"}
// Dependencies: {}
ast_enum ! { # [doc = " Data stored within an enum variant or struct."] pub enum VariantData { # [doc = " Struct variant, e.g. `Point { x: f64, y: f64 }`."] Struct (Delimited < Field , tokens :: Comma >, tokens :: Brace) , # [doc = " Tuple variant, e.g. `Some(T)`."] Tuple (Delimited < Field , tokens :: Comma >, tokens :: Paren) , # [doc = " Unit variant, e.g. `None`."] Unit , } }
};
}
