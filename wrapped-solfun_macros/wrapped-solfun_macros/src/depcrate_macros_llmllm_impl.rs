// Generated macro for llm_impl (function)
macro_rules! Depcrate_macros_llmllm_impl {
() => {
// Module: crate::macros::llm
// Provides: {"llm_impl"}
// Dependencies: {}
# [decl (fn , name = "llm_impl" , vis = "pub" , hash = "ec706832")] pub fn llm_impl (input : TokenStream) -> TokenStream { let request = parse_macro_input ! (input as LitStr) ; let request_content = request . value () ; let span = request . span () ; let tool_name = "llm_text_generation_service" ; let tool_args = format ! ("request: \"{}\"" , request_content) ; quote_spanned ! { span => eprintln ! ("\n🧠 LLM! Interpreted request: \"{{}}\". Resolving to toolcall!\n" , # request) ; solfun_macros :: toolcall ! { { # tool_name , # tool_args } } () } . into () }
};
}
