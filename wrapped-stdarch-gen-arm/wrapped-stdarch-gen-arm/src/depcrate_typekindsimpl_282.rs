// Generated macro for impl_282 (impl)
macro_rules! Depcrate_typekindsimpl_282 {
() => {
// Module: crate::typekinds
// Provides: {"impl_282"}
// Dependencies: {}
impl FromStr for VectorType { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { static RE : LazyLock < Regex > = LazyLock :: new (| | { Regex :: new (r"^(?:(?:sv(?P<sv_ty>(?:uint|int|bool|float)(?:\d+)?))|(?:(?P<ty>(?:uint|int|bool|poly|float)(?:\d+)?)x(?P<lanes>(?:\d+)?)))(?:x(?P<tuple_size>2|3|4))?_t$") . unwrap () }) ; if let Some (c) = RE . captures (s) { let (base_type , lanes) = Self :: sanitise_lanes (c . name ("sv_ty") . or_else (| | c . name ("ty")) . map (< & str > :: from) . map (BaseType :: from_str) . unwrap () ? , c . name ("lanes") . map (< & str > :: from) . map (u32 :: from_str) . transpose () . unwrap () ,) . map_err (| e | format ! ("invalid {s:#?} vector type: {e}")) ? ; let tuple_size = c . name ("tuple_size") . map (< & str > :: from) . map (VectorTupleSize :: from_str) . transpose () . unwrap () ; Ok (VectorType { base_type , is_scalable : c . name ("sv_ty") . is_some () , lanes , tuple_size , }) } else { Err (format ! ("invalid vector type {s:#?} given")) } } }
};
}
