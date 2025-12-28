macro_rules! QueryJobId {
    () => {
        # [doc = " A value uniquely identifying an active query job."] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct QueryJobId (pub NonZero < u64 >) ;
    };
}

QueryJobId!()