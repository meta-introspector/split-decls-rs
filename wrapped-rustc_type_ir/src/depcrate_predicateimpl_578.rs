// Generated macro for impl_578 (impl)
macro_rules! Depcrate_predicateimpl_578 {
() => {
// Module: crate::predicate
// Provides: {"impl_578"}
// Dependencies: {}
impl BoundConstness { pub fn satisfies (self , goal : BoundConstness) -> bool { match (self , goal) { (BoundConstness :: Const , BoundConstness :: Const | BoundConstness :: Maybe) => true , (BoundConstness :: Maybe , BoundConstness :: Maybe) => true , (BoundConstness :: Maybe , BoundConstness :: Const) => false , } } pub fn as_str (self) -> & 'static str { match self { Self :: Const => "const" , Self :: Maybe => "[const]" , } } }
};
}
