// Generated macro for int_to_int (macro)
macro_rules! Depcrate_de_implsint_to_int {
() => {
// Module: crate::de::impls
// Provides: {"int_to_int"}
// Dependencies: {}
macro_rules ! int_to_int { ($ ty : ident : $ visit : ident) => { # [inline] fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if Self :: Value :: min_value () as i64 <= v as i64 && v as i64 <= Self :: Value :: max_value () as i64 { Ok (v as Self :: Value) } else { Err (Error :: invalid_value (Unexpected :: Signed (v as i64) , & self)) } } } ; (nonzero $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if $ primitive :: min_value () as i64 <= v as i64 && v as i64 <= $ primitive :: max_value () as i64 { if let Some (nonzero) = Self :: Value :: new (v as $ primitive) { return Ok (nonzero) ; } } Err (Error :: invalid_value (Unexpected :: Signed (v as i64) , & self)) } } ; (saturating $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if (v as i64) < $ primitive :: MIN as i64 { Ok (Saturating ($ primitive :: MIN)) } else if ($ primitive :: MAX as i64) < v as i64 { Ok (Saturating ($ primitive :: MAX)) } else { Ok (Saturating (v as $ primitive)) } } } ; }
};
}
