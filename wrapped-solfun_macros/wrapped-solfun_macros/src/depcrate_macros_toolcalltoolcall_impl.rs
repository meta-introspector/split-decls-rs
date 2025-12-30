// Generated macro for toolcall_impl (function)
macro_rules! Depcrate_macros_toolcalltoolcall_impl {
() => {
// Module: crate::macros::toolcall
// Provides: {"toolcall_impl"}
// Dependencies: {}
# [decl (fn , name = "toolcall_impl" , vis = "pub" , hash = "c02aaffd")] pub fn toolcall_impl (input : TokenStream) -> TokenStream { let args = parse_macro_input ! (input as ToolCallArgs) ; let tool_name = args . tool_name ; let args_value = args . args ; let span = tool_name . span () ; let simulated_tool_output = format ! ("Tool '{}' executed with args: {}" , tool_name . value () , args_value . value ()) ; quote_spanned ! { span => eprintln ! ("\n🛠️ TOOLCALL! Executing tool: `{}` with args: `{}`. Awaiting results...\n" , # tool_name , # args_value) ; solfun_macros :: results ! { { # simulated_tool_output } } () } . into () }
};
}
