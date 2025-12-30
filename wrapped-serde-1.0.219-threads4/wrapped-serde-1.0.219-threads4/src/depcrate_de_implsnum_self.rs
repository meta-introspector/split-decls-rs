// Generated macro for num_self (macro)
macro_rules! Depcrate_de_implsnum_self {
() => {
// Module: crate::de::impls
// Provides: {"num_self"}
// Dependencies: {}
macro_rules ! num_self { ($ ty : ident : $ visit : ident) => { # [inline] fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { Ok (v) } } ; (nonzero $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { if let Some (nonzero) = Self :: Value :: new (v) { Ok (nonzero) } else { Err (Error :: invalid_value (Unexpected :: Unsigned (0) , & self)) } } } ; (saturating $ primitive : ident $ ty : ident : $ visit : ident) => { fn $ visit < E > (self , v : $ ty) -> Result < Self :: Value , E > where E : Error , { Ok (Saturating (v)) } } ; }
};
}
