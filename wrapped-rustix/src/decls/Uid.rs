macro_rules! deps {
    () => {
        RawUid!();
    };
}

macro_rules! Uid {
    () => {
        deps!();
        # [doc = " `uid_t`—A Unix user ID."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Debug , Hash)] pub struct Uid (RawUid) ;
    };
}

Uid!();