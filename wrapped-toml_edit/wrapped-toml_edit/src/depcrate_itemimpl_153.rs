// Generated macro for impl_153 (impl)
macro_rules! Depcrate_itemimpl_153 {
() => {
// Module: crate::item
// Provides: {"impl_153"}
// Dependencies: {}
# [cfg (feature = "parse")] impl FromStr for Item { type Err = crate :: TomlError ; # [doc = " Parses a value from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let value = s . parse :: < Value > () ? ; Ok (Self :: Value (value)) } }
};
}
