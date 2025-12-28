macro_rules! deps {
    () => {
        StaticTreeDesc!();
        Value!();
    };
}

macro_rules! TreeDesc {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct TreeDesc < const N : usize > { dyn_tree : [Value ; N] , max_code : usize , stat_desc : & 'static StaticTreeDesc , }
    };
}

TreeDesc!();