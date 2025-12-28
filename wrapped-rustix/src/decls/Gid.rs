macro_rules! deps {
    () => {
        RawGid!();
    };
}

macro_rules! Gid {
    () => {
        deps!();
        # [doc = " `gid_t`—A Unix group ID."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Debug , Hash)] pub struct Gid (RawGid) ;
    };
}

Gid!()