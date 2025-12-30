// Generated macro for empty_visit (macro)
macro_rules! Depcrate_visitempty_visit {
() => {
// Module: crate::visit
// Provides: {"empty_visit"}
// Dependencies: {}
macro_rules ! empty_visit { ($ name : ident , $ t : ty) => { fn $ name <'doc , V > (_v : & mut V , _node : &'doc $ t) where V : Visit <'doc > + ? Sized , { } } ; }
};
}
