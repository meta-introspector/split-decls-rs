// Generated macro for Module (struct)
macro_rules! Depcrate_modulesModule {
() => {
// Module: crate::modules
// Provides: {"Module"}
// Dependencies: {}
# [doc = " Represents module with its inner attributes."] # [derive (Debug , Clone)] pub (crate) struct Module < 'a > { ast_mod_kind : Option < Cow < 'a , ast :: ModKind > > , pub (crate) items : Cow < 'a , ThinVec < rustc_ast :: ptr :: P < ast :: Item > > > , inner_attr : ast :: AttrVec , pub (crate) span : Span , }
};
}
