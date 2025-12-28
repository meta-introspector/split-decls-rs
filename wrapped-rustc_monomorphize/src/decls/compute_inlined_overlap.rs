macro_rules! compute_inlined_overlap {
    () => {
        # [doc = " Compute the combined size of all inlined items that appear in both `cgu1`"] # [doc = " and `cgu2`."] fn compute_inlined_overlap < 'tcx > (cgu1 : & CodegenUnit < 'tcx > , cgu2 : & CodegenUnit < 'tcx >) -> usize { let (src_cgu , dst_cgu) = if cgu1 . items () . len () <= cgu2 . items () . len () { (cgu1 , cgu2) } else { (cgu2 , cgu1) } ; let mut overlap = 0 ; for (item , data) in src_cgu . items () . iter () { if data . inlined && dst_cgu . items () . contains_key (item) { overlap += data . size_estimate ; } } overlap }
    };
}

compute_inlined_overlap!();