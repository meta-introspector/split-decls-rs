// Generated macro for impl_92 (impl)
macro_rules! Depcrateimpl_92 {
() => {
// Module: crate
// Provides: {"impl_92"}
// Dependencies: {}
impl < Ctx : fmt :: Debug > fmt :: Debug for SpanData < Ctx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { fmt :: Debug :: fmt (& self . anchor . file_id . file_id () . index () , f) ? ; f . write_char (':') ? ; write ! (f , "{:#?}" , self . anchor . ast_id) ? ; f . write_char ('@') ? ; fmt :: Debug :: fmt (& self . range , f) ? ; f . write_char ('#') ? ; self . ctx . fmt (f) } else { f . debug_struct ("SpanData") . field ("range" , & self . range) . field ("anchor" , & self . anchor) . field ("ctx" , & self . ctx) . finish () } } }
};
}
