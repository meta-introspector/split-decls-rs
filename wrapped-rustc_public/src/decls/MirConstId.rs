macro_rules! MirConstId {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct MirConstId (usize) ;
    };
}

MirConstId!()