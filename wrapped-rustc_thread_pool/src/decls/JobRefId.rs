macro_rules! JobRefId {
    () => {
        # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub (super) struct JobRefId { pointer : usize , }
    };
}

JobRefId!()