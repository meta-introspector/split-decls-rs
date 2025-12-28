macro_rules! deps {
    () => {
        QueryRevisionsExtraInner!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        # [cfg (not (feature = "shuttle"))] # [cfg (target_pointer_width = "64")] const _ : [() ; std :: mem :: size_of :: < QueryRevisionsExtraInner > ()] = [() ; std :: mem :: size_of :: < [usize ; if cfg ! (feature = "accumulator") { 7 } else { 3 }] > ()] ;
    };
}

_!();