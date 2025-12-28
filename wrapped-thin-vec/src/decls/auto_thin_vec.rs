macro_rules! deps {
    () => {
        AutoThinVec!();
    };
}

macro_rules! auto_thin_vec {
    () => {
        deps!();
        # [doc = " Create a ThinVec<$ty> named `$name`, with capacity for `$cap` inline elements."] # [doc = ""] # [doc = " TODO(emilio): This would be a lot more convenient to use with super let, see"] # [doc = " <https://github.com/rust-lang/rust/issues/139076>"] # [cfg (feature = "gecko-ffi")] # [macro_export] macro_rules ! auto_thin_vec { (let $ name : ident : [$ ty : ty ; $ cap : literal]) => { let auto_vec = $ crate :: AutoThinVec ::<$ ty , $ cap >:: new_unpinned () ; let mut $ name = core :: pin :: pin ! (auto_vec) ; unsafe { $ name . as_mut () . shrink_to_fit_known_singleton () } ; } ; }
    };
}

auto_thin_vec!()