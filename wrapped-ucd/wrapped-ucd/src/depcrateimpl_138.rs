// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
impl CharIter { pub fn new (osl : Option < & 'static [(u8 , u8 , u8)] > , cp : char) -> CharIter { CharIter (match osl { Some (sl) => CharIterInternal :: Iterator (sl . iter ()) , None => CharIterInternal :: Single (cp) }) } pub fn hangul (a : char , b : char) -> CharIter { CharIter (CharIterInternal :: Double (a , b)) } }
};
}
