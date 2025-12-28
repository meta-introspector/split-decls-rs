macro_rules! deps {
    () => {
        BoundTyKind!();
    };
}

macro_rules! BoundTy {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct BoundTy { pub var : usize , pub kind : BoundTyKind , }
    };
}

BoundTy!()