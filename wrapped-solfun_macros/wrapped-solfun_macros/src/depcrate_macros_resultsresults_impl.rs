// Generated macro for results_impl (function)
macro_rules! Depcrate_macros_resultsresults_impl {
() => {
// Module: crate::macros::results
// Provides: {"results_impl"}
// Dependencies: {}
# [decl (fn , name = "results_impl" , vis = "pub" , hash = "c1a27a24")] pub fn results_impl (input : TokenStream) -> TokenStream { let output_literal = parse_macro_input ! (input as LitStr) ; let span = output_literal . span () ; let encoded_vector_representation = format ! ("Vector encoding of: '{}'" , output_literal . value ()) ; quote_spanned ! { span => eprintln ! ("\n📊 RESULTS! Output received: '{}'. Encoded to vector: '{}'.\n" , # output_literal , # encoded_vector_representation) ; () } . into () }
};
}
