// Generated macro for impl_98 (impl)
macro_rules! Depcrate_errorimpl_98 {
() => {
// Module: crate::error
// Provides: {"impl_98"}
// Dependencies: {}
impl < C > ContextError < C > { # [doc = " Create an empty error"] # [inline] pub fn new () -> Self { Self { context : Default :: default () , # [cfg (feature = "std")] cause : None , } } # [doc = " Add more context"] # [inline] pub fn push (& mut self , context : C) { # [cfg (feature = "alloc")] self . context . push (context) ; } # [doc = " Add more context"] # [inline] pub fn extend < I : IntoIterator < Item = C > > (& mut self , context : I) { # [cfg (feature = "alloc")] self . context . extend (context) ; } # [doc = " Access context from [`Parser::context`]"] # [inline] # [cfg (feature = "alloc")] pub fn context (& self) -> impl Iterator < Item = & C > { self . context . iter () } # [doc = " Originating [`std::error::Error`]"] # [inline] # [cfg (feature = "std")] pub fn cause (& self) -> Option < & (dyn std :: error :: Error + Send + Sync + 'static) > { self . cause . as_deref () } }
};
}
