macro_rules! TransmuteRefMutDstOffsetIncrease {
    () => {
        # [doc = " It's not possible in the general case to increase the trailing slice offset"] # [doc = " during a reference transmutation - some pointer metadata values would not be"] # [doc = " supportable, and so such a transmutation would be fallible."] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " use zerocopy::doctests::SliceDst;"] # [doc = " let src: &SliceDst<u8, u8> = SliceDst::new();"] # [doc = " let increase_offset: &SliceDst<[u8; 2], u8> = zerocopy::transmute_ref!(src);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " use zerocopy::doctests::SliceDst;"] # [doc = " let src: &mut SliceDst<u8, u8> = SliceDst::new_mut();"] # [doc = " let increase_offset: &mut SliceDst<[u8; 2], u8> = zerocopy::transmute_mut!(src);"] # [doc = " ```"] enum TransmuteRefMutDstOffsetIncrease { }
    };
}

TransmuteRefMutDstOffsetIncrease!();