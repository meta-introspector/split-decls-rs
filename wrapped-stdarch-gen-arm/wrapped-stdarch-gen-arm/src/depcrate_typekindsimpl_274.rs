// Generated macro for impl_274 (impl)
macro_rules! Depcrate_typekindsimpl_274 {
() => {
// Module: crate::typekinds
// Provides: {"impl_274"}
// Dependencies: {}
impl FromStr for TypeKind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (match s { s if s . starts_with ('{') && s . ends_with ('}') => { Self :: Wildcard (s [1 .. s . len () - 1] . trim () . parse () ?) } s if s . starts_with ('*') => { let mut split = s [1 ..] . split_whitespace () ; let (ty , rw) = match (split . clone () . count () , split . next () , split . next ()) { (2 , Some ("mut") , Some (ty)) => (ty , AccessLevel :: RW) , (2 , Some ("const") , Some (ty)) => (ty , AccessLevel :: R) , (1 , Some (ty) , None) => (ty , AccessLevel :: R) , _ => return Err (format ! ("invalid pointer type {s:#?} given")) , } ; Self :: Pointer (Box :: new (ty . parse () ?) , rw) } _ => s . parse :: < VectorType > () . map (TypeKind :: Vector) . or_else (| _ | s . parse :: < BaseType > () . map (TypeKind :: Base)) . unwrap_or_else (| _ | TypeKind :: Custom (s . to_string ())) , }) } }
};
}
