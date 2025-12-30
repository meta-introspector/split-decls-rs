// Generated macro for impl_1086 (impl)
macro_rules! Depcrate_combinator_implsimpl_1086 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1086"}
// Dependencies: {}
impl < F , I , O , E > Parser < I , (O , Range < usize >) , E > for WithSpan < F , I , O , E > where F : Parser < I , O , E > , I : Stream + Location , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < (O , Range < usize >) , E > { let start = input . current_token_start () ; self . parser . parse_next (input) . map (move | output | { let end = input . previous_token_end () ; (output , (start .. end)) }) } }
};
}
