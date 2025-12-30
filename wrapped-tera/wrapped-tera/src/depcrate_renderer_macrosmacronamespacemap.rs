// Generated macro for MacroNamespaceMap (type)
macro_rules! Depcrate_renderer_macrosMacroNamespaceMap {
() => {
// Module: crate::renderer::macros
// Provides: {"MacroNamespaceMap"}
// Dependencies: {}
# [doc = " Maps { namespace => ( macro_template, { macro => macro_definition }) }"] pub type MacroNamespaceMap < 'a > = HashMap < & 'a str , (& 'a str , & 'a MacroDefinitionMap) > ;
};
}
