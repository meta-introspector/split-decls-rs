// Generated macro for impl_293 (impl)
macro_rules! Depcrate_typekindsimpl_293 {
() => {
// Module: crate::typekinds
// Provides: {"impl_293"}
// Dependencies: {}
impl FromStr for BaseType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { static RE : LazyLock < Regex > = LazyLock :: new (| | Regex :: new (r"^(?P<kind>[a-zA-Z]+)(?P<size>\d+)?(_t)?$") . unwrap ()) ; if let Some (c) = RE . captures (s) { let kind = c ["kind"] . parse () ? ; let size = c . name ("size") . map (< & str > :: from) . map (u32 :: from_str) . transpose () . unwrap () ; match size { Some (size) => Ok (Self :: Sized (kind , size)) , None => Ok (Self :: Unsized (kind)) , } } else { Err (format ! ("failed to parse type `{s}`")) } } }
};
}
