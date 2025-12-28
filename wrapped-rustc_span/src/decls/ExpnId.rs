macro_rules! ExpnId {
    () => {
        # [doc = " A unique ID associated with a macro invocation and expansion."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct ExpnId { pub krate : CrateNum , pub local_id : ExpnIndex , }
    };
}

ExpnId!();