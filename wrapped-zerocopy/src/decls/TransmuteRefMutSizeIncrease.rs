macro_rules! TransmuteRefMutSizeIncrease {
    () => {
        # [doc = " We require that the size of the destination type is not larger than the size"] # [doc = " of the source type."] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " let increase_size: &[u8; 2] = zerocopy::transmute_ref!(&0u8);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " let mut src = 0u8;"] # [doc = " let increase_size: &mut [u8; 2] = zerocopy::transmute_mut!(&mut src);"] # [doc = " ```"] enum TransmuteRefMutSizeIncrease { }
    };
}

TransmuteRefMutSizeIncrease!();