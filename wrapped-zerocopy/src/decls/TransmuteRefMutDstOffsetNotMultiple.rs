macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! TransmuteRefMutDstOffsetNotMultiple {
    () => {
        deps!();
        # [doc = " Reference transmutes are not possible when the difference between the source"] # [doc = " and destination types' trailing slice offsets is not a multiple of the"] # [doc = " destination type's trailing slice element size."] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " use zerocopy::doctests::SliceDst;"] # [doc = " let src: &SliceDst<[u8; 3], [u8; 2]> = SliceDst::new();"] # [doc = " let _: &SliceDst<[u8; 2], [u8; 2]> = zerocopy::transmute_ref!(src);"] # [doc = " ```"] # [doc = ""] # [doc = " ```compile_fail,E0080"] # [doc = " use zerocopy::doctests::SliceDst;"] # [doc = " let src: &mut SliceDst<[u8; 3], [u8; 2]> = SliceDst::new_mut();"] # [doc = " let _: &mut SliceDst<[u8; 2], [u8; 2]> = zerocopy::transmute_mut!(src);"] # [doc = " ```"] enum TransmuteRefMutDstOffsetNotMultiple { }
    };
}

TransmuteRefMutDstOffsetNotMultiple!()