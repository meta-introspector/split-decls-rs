macro_rules! deps {
    () => {
        Type!();
        Region!();
    };
}

macro_rules! Reference {
    () => {
        deps!();
        # [doc = " A reference, i.e., `&'region T` or `&'region mut T`."] # [derive (Debug , Hash , Eq , PartialEq , Ord , PartialOrd , Clone , Copy)] pub (crate) struct Reference < R , T > where R : Region , T : Type , { pub (crate) region : R , pub (crate) is_mut : bool , pub (crate) referent : T , pub (crate) referent_size : usize , pub (crate) referent_align : usize , }
    };
}

Reference!()