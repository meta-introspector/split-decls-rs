// Generated macro for Contexts (struct)
macro_rules! Depcrate_cldr_serde_caContexts {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"Contexts"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Deserialize)] pub (crate) struct Contexts < Symbols > { pub (crate) format : FormatWidths < Symbols > , # [serde (rename = "stand-alone")] pub (crate) stand_alone : Option < StandAloneWidths < Symbols > > , pub (crate) numeric : Option < Numeric < Symbols > > , }
};
}
