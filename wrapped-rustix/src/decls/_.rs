macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        # [cfg (any (target_pointer_width = "16" , target_pointer_width = "32" , target_pointer_width = "64"))] const _ : () = { impl Integer for isize { } impl Integer for usize { } } ;
    };
}

_!()