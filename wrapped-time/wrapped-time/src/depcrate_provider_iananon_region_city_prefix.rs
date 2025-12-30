// Generated macro for NON_REGION_CITY_PREFIX (const)
macro_rules! Depcrate_provider_ianaNON_REGION_CITY_PREFIX {
() => {
// Module: crate::provider::iana
// Provides: {"NON_REGION_CITY_PREFIX"}
// Dependencies: {}
# [doc = " [`IanaToBcp47Map`]'s trie cannot handle differently-cased prefixes, like `Mexico/BajaSur`` and `MET`."] # [doc = ""] # [doc = " Therefore, any ID that is not of the shape `{region}/{city}` gets prefixed with this character"] # [doc = " inside the trie."] # [doc = ""] # [doc = " During lookup, if the input is not of the shape `{region}/{city}`, the trie cursor has to be advanced over"] # [doc = " this byte."] pub const NON_REGION_CITY_PREFIX : u8 = b'_' ;
};
}
