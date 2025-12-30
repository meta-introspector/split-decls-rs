// Generated macro for uint_to_self (macro)
macro_rules! Depcrate_de_implsuint_to_self {
() => {
// Module: crate::de::impls
// Provides: {"uint_to_self"}
// Dependencies: {}
macro_rules ! uint_to_self { ($ ty : ident : $ visit : ident) => { # [inline] fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if v as u64 <= Self :: Value :: max_value () as u64 { Ok (v as Self :: Value) } else { Err (Error :: invalid_value (Unexpected :: Unsigned (v as u64) , & self)) } } } ; (nonzero $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if v as u64 <= $ primitive :: max_value () as u64 { if let Some (nonzero) = Self :: Value :: new (v as $ primitive) { return Ok (nonzero) ; } } Err (Error :: invalid_value (Unexpected :: Unsigned (v as u64) , & self)) } } ; (saturating $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if v as u64 <= $ primitive :: MAX as u64 { Ok (Saturating (v as $ primitive)) } else { Ok (Saturating ($ primitive :: MAX)) } } } ; }
};
}
