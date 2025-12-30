// Generated macro for impl_289 (impl)
macro_rules! Depcrate_typekindsimpl_289 {
() => {
// Module: crate::typekinds
// Provides: {"impl_289"}
// Dependencies: {}
impl FromStr for BaseTypeKind { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "float" | "f" => Ok (Self :: Float) , "int" | "i" => Ok (Self :: Int) , "uint" | "u" => Ok (Self :: UInt) , "poly" | "p" => Ok (Self :: Poly) , "bool" | "b" => Ok (Self :: Bool) , _ => Err (format ! ("no match for {s}")) , } } }
};
}
