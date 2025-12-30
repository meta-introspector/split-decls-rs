// Generated macro for int_to_uint (macro)
macro_rules! Depcrate_de_implsint_to_uint {
() => {
// Module: crate::de::impls
// Provides: {"int_to_uint"}
// Dependencies: {}
macro_rules ! int_to_uint { ($ ty : ident : $ visit : ident) => { # [inline] fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if 0 <= v && v as u64 <= Self :: Value :: max_value () as u64 { Ok (v as Self :: Value) } else { Err (Error :: invalid_value (Unexpected :: Signed (v as i64) , & self)) } } } ; (nonzero $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if 0 < v && v as u64 <= $ primitive :: max_value () as u64 { if let Some (nonzero) = Self :: Value :: new (v as $ primitive) { return Ok (nonzero) ; } } Err (Error :: invalid_value (Unexpected :: Signed (v as i64) , & self)) } } ; (saturating $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if v < 0 { Ok (Saturating (0)) } else if ($ primitive :: MAX as u64) < v as u64 { Ok (Saturating ($ primitive :: MAX)) } else { Ok (Saturating (v as $ primitive)) } } } ; }
};
}
