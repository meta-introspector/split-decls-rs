// Generated macro for impl_42 (impl)
macro_rules! Depcrate_delimitedimpl_42 {
() => {
// Module: crate::delimited
// Provides: {"impl_42"}
// Dependencies: {}
impl < T , D > Element < T , D > { pub fn into_item (self) -> T { match self { Element :: Delimited (t , _) | Element :: End (t) => t , } } pub fn item (& self) -> & T { match * self { Element :: Delimited (ref t , _) | Element :: End (ref t) => t , } } pub fn item_mut (& mut self) -> & mut T { match * self { Element :: Delimited (ref mut t , _) | Element :: End (ref mut t) => t , } } pub fn delimiter (& self) -> Option < & D > { match * self { Element :: Delimited (_ , ref d) => Some (d) , Element :: End (_) => None , } } pub fn into_tuple (self) -> (T , Option < D >) { match self { Element :: Delimited (t , d) => (t , Some (d)) , Element :: End (t) => (t , None) , } } }
};
}
