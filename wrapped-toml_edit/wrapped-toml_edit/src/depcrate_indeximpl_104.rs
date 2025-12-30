// Generated macro for impl_104 (impl)
macro_rules! Depcrate_indeximpl_104 {
() => {
// Module: crate::index
// Provides: {"impl_104"}
// Dependencies: {}
impl Index for usize { fn index < 'v > (& self , v : & 'v Item) -> Option < & 'v Item > { match * v { Item :: ArrayOfTables (ref aot) => aot . values . get (* self) , Item :: Value (ref a) if a . is_array () => a . as_array () . and_then (| a | a . values . get (* self)) , _ => None , } } fn index_mut < 'v > (& self , v : & 'v mut Item) -> Option < & 'v mut Item > { match * v { Item :: ArrayOfTables (ref mut vec) => vec . values . get_mut (* self) , Item :: Value (ref mut a) => a . as_array_mut () . and_then (| a | a . values . get_mut (* self)) , _ => None , } } }
};
}
