// Generated macro for num_128 (macro)
macro_rules! Depcrate_de_implsnum_128 {
() => {
// Module: crate::de::impls
// Provides: {"num_128"}
// Dependencies: {}
macro_rules ! num_128 { ($ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if v as i128 >= Self :: Value :: min_value () as i128 && v as u128 <= Self :: Value :: max_value () as u128 { Ok (v as Self :: Value) } else { Err (Error :: invalid_value (Unexpected :: Other (stringify ! ($ ty)) , & self ,)) } } } ; (nonzero $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if v as i128 >= $ primitive :: min_value () as i128 && v as u128 <= $ primitive :: max_value () as u128 { if let Some (nonzero) = Self :: Value :: new (v as $ primitive) { Ok (nonzero) } else { Err (Error :: invalid_value (Unexpected :: Unsigned (0) , & self)) } } else { Err (Error :: invalid_value (Unexpected :: Other (stringify ! ($ ty)) , & self ,)) } } } ; (saturating $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if (v as i128) < $ primitive :: MIN as i128 { Ok (Saturating ($ primitive :: MIN)) } else if ($ primitive :: MAX as u128) < v as u128 { Ok (Saturating ($ primitive :: MAX)) } else { Ok (Saturating (v as $ primitive)) } } } ; }
};
}
