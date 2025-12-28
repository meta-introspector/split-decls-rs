macro_rules! CtorKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Serialize)] pub enum CtorKind { Const , Fn , }
    };
}

CtorKind!()