macro_rules! TyConstId {
    () => {
        # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct TyConstId (usize) ;
    };
}

TyConstId!()