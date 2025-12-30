// Generated macro for NumberingSystem (struct)
macro_rules! Depcrate_cldr_serde_numbering_systemsNumberingSystem {
() => {
// Module: crate::cldr_serde::numbering_systems
// Provides: {"NumberingSystem"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct NumberingSystem { # [serde (rename = "_type")] pub (crate) nstype : NumberingSystemType , # [serde (rename = "_digits")] pub (crate) digits : Option < String > , # [serde (rename = "_rules")] pub (crate) rules : Option < String > , }
};
}
