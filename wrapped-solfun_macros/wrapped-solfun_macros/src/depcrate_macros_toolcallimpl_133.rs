// Generated macro for impl_133 (impl)
macro_rules! Depcrate_macros_toolcallimpl_133 {
() => {
// Module: crate::macros::toolcall
// Provides: {"impl_133"}
// Dependencies: {}
impl Parse for ToolCallArgs { fn parse (input : ParseStream) -> SynResult < Self > { let tool_name = input . parse () ? ; let _comma = input . parse () ? ; let args = input . parse () ? ; Ok (ToolCallArgs { tool_name , _comma , args }) } }
};
}
