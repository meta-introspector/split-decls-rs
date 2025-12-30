// Generated macro for asm_concat (function)
macro_rules! Depcrate_testsasm_concat {
() => {
// Module: crate::tests
// Provides: {"asm_concat"}
// Dependencies: {}
# [test] fn asm_concat () { let asm_pre = r###"concat!("invalid", "_", "instruction")"### ; let asm = "invalid_instruction" ; let mut parser = Parser :: new (asm , None , Some (asm_pre . into ()) , false , ParseMode :: InlineAsm) ; assert ! (! parser . is_source_literal) ; assert_eq ! (parser . by_ref () . collect ::< Vec < Piece <'static >>> () , & [Lit (asm)]) ; assert_eq ! (parser . line_spans , & []) ; }
};
}
