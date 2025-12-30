// Generated macro for impl_1280 (impl)
macro_rules! Depcrate_skipimpl_1280 {
() => {
// Module: crate::skip
// Provides: {"impl_1280"}
// Dependencies: {}
impl SkipNameContext { pub (crate) fn update (& mut self , other : Self) { match (self , other) { (Self :: All , _) => { } (this , Self :: All) => { * this = Self :: All ; } (Self :: Values (existing_values) , Self :: Values (new_values)) => { existing_values . extend (new_values) } } } pub (crate) fn skip (& self , name : & str) -> bool { match self { Self :: All => true , Self :: Values (values) => values . contains (name) , } } pub (crate) fn skip_all (& mut self) { * self = Self :: All ; } }
};
}
