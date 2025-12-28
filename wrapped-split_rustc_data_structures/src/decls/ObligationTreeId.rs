macro_rules! ObligationTreeId {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] struct ObligationTreeId (usize) ;
    };
}

ObligationTreeId!();