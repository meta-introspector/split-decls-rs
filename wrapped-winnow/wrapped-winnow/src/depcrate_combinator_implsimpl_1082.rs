// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_combinator_implsimpl_1082 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1082"}
// Dependencies: {}
impl < F , I , O , E > Parser < I , (O , < I as Stream > :: Slice) , E > for WithTaken < F , I , O , E > where F : Parser < I , O , E > , I : Stream , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < (O , < I as Stream > :: Slice) , E > { let checkpoint = input . checkpoint () ; match (self . parser) . parse_next (input) { Ok (result) => { let offset = input . offset_from (& checkpoint) ; input . reset (& checkpoint) ; let taken = input . next_slice (offset) ; Ok ((result , taken)) } Err (e) => Err (e) , } } }
};
}
