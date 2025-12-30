// Generated macro for weak (macro)
macro_rules! Depcrate_weakweak {
() => {
// Module: crate::weak
// Provides: {"weak"}
// Dependencies: {}
macro_rules ! weak { ($ vis : vis fn $ name : ident ($ ($ t : ty) ,*) -> $ ret : ty) => (# [allow (non_upper_case_globals)] $ vis static $ name : $ crate :: weak :: Weak < unsafe extern "C" fn ($ ($ t) ,*) -> $ ret > = $ crate :: weak :: Weak :: new (concat ! (stringify ! ($ name) , '\0')) ;) }
};
}
