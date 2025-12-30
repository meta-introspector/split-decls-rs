// Generated macro for stringify_outputs (macro)
macro_rules! Depcrate_convertstringify_outputs {
() => {
// Module: crate::convert
// Provides: {"stringify_outputs"}
// Dependencies: {}
# [doc = " Given the list of types, stringify them as a list."] macro_rules ! stringify_outputs { (@ inner $ first : ty) => { concat ! ("or `" , stringify ! ($ first) , "`") } ; (@ inner $ first : ty , $ ($ t : ty) ,+) => { concat ! (stringify_outputs ! ($ first) , ", " , stringify_outputs ! (@ inner $ ($ t) ,+)) } ; ($ first : ty) => { concat ! ("`" , stringify ! ($ first) , "`") } ; ($ ($ t : ty) ,+) => { stringify_outputs ! (@ inner $ ($ t) ,+) } ; }
};
}
