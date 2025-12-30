// Generated macro for Alias (struct)
macro_rules! Depcrate_cldr_serde_aliasesAlias {
() => {
// Module: crate::cldr_serde::aliases
// Provides: {"Alias"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct Alias { # [serde (rename = "languageAlias")] pub (crate) language_aliases : HashMap < String , Replacement < String > > , # [serde (rename = "scriptAlias")] pub (crate) script_aliases : HashMap < TinyAsciiStr < 4 > , Replacement < TinyAsciiStr < 4 > > > , # [serde (rename = "territoryAlias")] pub (crate) region_aliases : HashMap < TinyAsciiStr < 3 > , Replacement < String > > , # [serde (rename = "variantAlias")] pub (crate) variant_aliases : HashMap < TinyAsciiStr < 8 > , Replacement < TinyAsciiStr < 8 > > > , # [serde (rename = "subdivisionAlias")] pub (crate) subdivision_aliases : HashMap < TinyAsciiStr < 7 > , Replacement < String > > , }
};
}
