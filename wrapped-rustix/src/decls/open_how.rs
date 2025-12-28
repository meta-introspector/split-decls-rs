macro_rules! open_how {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct open_how { # [doc = " An [`OFlags`] value represented as a `u64`."] pub flags : u64 , # [doc = " A [`Mode`] value represented as a `u64`."] pub mode : u64 , pub resolve : ResolveFlags , }
    };
}

open_how!();