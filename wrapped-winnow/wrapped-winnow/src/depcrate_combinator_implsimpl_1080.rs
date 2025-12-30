// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_combinator_implsimpl_1080 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1080"}
// Dependencies: {}
impl < I , O , E , F > Parser < I , < I as Stream > :: Slice , E > for Take < F , I , O , E > where F : Parser < I , O , E > , I : Stream , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < < I as Stream > :: Slice , E > { let checkpoint = input . checkpoint () ; match (self . parser) . parse_next (input) { Ok (_) => { let offset = input . offset_from (& checkpoint) ; input . reset (& checkpoint) ; let taken = input . next_slice (offset) ; Ok (taken) } Err (e) => Err (e) , } } }
};
}
