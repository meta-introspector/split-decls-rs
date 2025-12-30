// Generated macro for num_as_self (macro)
macro_rules! Depcrate_de_implsnum_as_self {
() => {
// Module: crate::de::impls
// Provides: {"num_as_self"}
// Dependencies: {}
macro_rules ! num_as_self { ($ ty : ident : $ visit : ident) => { # [inline] fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { Ok (v as Self :: Value) } } ; (nonzero $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if let Some (nonzero) = Self :: Value :: new (v as $ primitive) { Ok (nonzero) } else { Err (Error :: invalid_value (Unexpected :: Unsigned (0) , & self)) } } } ; (saturating $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { Ok (Saturating (v as $ primitive)) } } ; }
};
}
