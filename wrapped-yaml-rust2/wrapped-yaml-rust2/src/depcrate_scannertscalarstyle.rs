// Generated macro for TScalarStyle (enum)
macro_rules! Depcrate_scannerTScalarStyle {
() => {
// Module: crate::scanner
// Provides: {"TScalarStyle"}
// Dependencies: {}
# [doc = " The style as which the scalar was written in the YAML document."] # [derive (Clone , Copy , PartialEq , Debug , Eq)] pub enum TScalarStyle { # [doc = " A YAML plain scalar."] Plain , # [doc = " A YAML single quoted scalar."] SingleQuoted , # [doc = " A YAML double quoted scalar."] DoubleQuoted , # [doc = " A YAML literal block (`|` block)."] Literal , # [doc = " A YAML folded block (`>` block)."] Folded , }
};
}
