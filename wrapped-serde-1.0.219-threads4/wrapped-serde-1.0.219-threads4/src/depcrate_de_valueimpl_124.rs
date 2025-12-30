// Generated macro for impl_124 (impl)
macro_rules! Depcrate_de_valueimpl_124 {
() => {
// Module: crate::de::value
// Provides: {"impl_124"}
// Dependencies: {}
impl < 'de , I , E > MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , { fn next_pair (& mut self) -> Option < (First < I :: Item > , Second < I :: Item >) > { match self . iter . next () { Some (kv) => { self . count += 1 ; Some (private :: Pair :: split (kv)) } None => None , } } }
};
}
