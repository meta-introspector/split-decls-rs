// Generated macro for CollationSpecialPrimaries (struct)
macro_rules! Depcrate_collator_collator_serdeCollationSpecialPrimaries {
() => {
// Module: crate::collator::collator_serde
// Provides: {"CollationSpecialPrimaries"}
// Dependencies: {}
# [doc = " Serde counterpart for `CollationSpecialPrimaries`."] # [derive (serde :: Deserialize)] pub (crate) struct CollationSpecialPrimaries { # [doc = " Length always supposed to be 4"] pub (crate) last_primaries : Vec < u16 > , # [doc = " `Option` to support datagen with ICU4X 2.0.0-associated"] # [doc = " TOML from ICU4C tag icu4x/2025-05-01/77.x, which doesn't"] # [doc = " have this data."] # [doc = ""] # [doc = " For correct results, when CLDR is updated on the ICU4C"] # [doc = " side, icuexportdata must start including this data in"] # [doc = " the TOML."] pub (crate) compressible_bytes : Option < Vec < bool > > , pub (crate) numeric_primary : u8 , }
};
}
