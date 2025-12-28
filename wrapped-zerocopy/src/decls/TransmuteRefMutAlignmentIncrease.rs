macro_rules! TransmuteRefMutAlignmentIncrease {
    () => {
        # [doc = " We require that the alignment of the destination type is not larger than the"] # [doc = " alignment of the source type."] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " let increase_alignment: &u16 = zerocopy::transmute_ref!(&[0u8; 2]);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " let mut src = [0u8; 2];"] # [doc = " let increase_alignment: &mut u16 = zerocopy::transmute_mut!(&mut src);"] # [doc = " ```"] enum TransmuteRefMutAlignmentIncrease { }
    };
}

TransmuteRefMutAlignmentIncrease!();