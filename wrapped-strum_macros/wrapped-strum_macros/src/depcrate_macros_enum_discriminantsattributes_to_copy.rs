// Generated macro for ATTRIBUTES_TO_COPY (const)
macro_rules! Depcrate_macros_enum_discriminantsATTRIBUTES_TO_COPY {
() => {
// Module: crate::macros::enum_discriminants
// Provides: {"ATTRIBUTES_TO_COPY"}
// Dependencies: {}
# [doc = " Attributes to copy from the main enum's variants to the discriminant enum's variants."] # [doc = ""] # [doc = " Attributes not in this list may be for other `proc_macro`s on the main enum, and may cause"] # [doc = " compilation problems when copied across."] const ATTRIBUTES_TO_COPY : & [& str] = & ["doc" , "cfg" , "allow" , "deny" , "strum_discriminants"] ;
};
}
