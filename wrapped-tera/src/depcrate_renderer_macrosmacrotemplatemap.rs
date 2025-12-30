// Generated macro for MacroTemplateMap (type)
macro_rules! Depcrate_renderer_macrosMacroTemplateMap {
() => {
// Module: crate::renderer::macros
// Provides: {"MacroTemplateMap"}
// Dependencies: {}
# [doc = " Maps { template => { namespace => ( macro_template, { macro => macro_definition }) }"] pub type MacroTemplateMap < 'a > = HashMap < & 'a str , MacroNamespaceMap < 'a > > ;
};
}
