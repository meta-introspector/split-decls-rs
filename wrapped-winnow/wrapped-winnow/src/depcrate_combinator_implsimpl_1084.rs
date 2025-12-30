// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_combinator_implsimpl_1084 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1084"}
// Dependencies: {}
impl < I , O , E , F > Parser < I , Range < usize > , E > for Span < F , I , O , E > where F : Parser < I , O , E > , I : Stream + Location , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < Range < usize > , E > { let start = input . current_token_start () ; self . parser . parse_next (input) . map (move | _ | { let end = input . previous_token_end () ; start .. end }) } }
};
}
