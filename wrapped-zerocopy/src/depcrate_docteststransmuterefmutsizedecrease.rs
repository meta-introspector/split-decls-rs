// Generated macro for TransmuteRefMutSizeDecrease (enum)
macro_rules! Depcrate_doctestsTransmuteRefMutSizeDecrease {
() => {
// Module: crate::doctests
// Provides: {"TransmuteRefMutSizeDecrease"}
// Dependencies: {}
# [doc = " We require that the size of the destination type is not smaller than the"] # [doc = " size of the source type."] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " let decrease_size: &u8 = zerocopy::transmute_ref!(&[0u8; 2]);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " let mut src = [0u8; 2];"] # [doc = " let decrease_size: &mut u8 = zerocopy::transmute_mut!(&mut src);"] # [doc = " ```"] enum TransmuteRefMutSizeDecrease { }
};
}
