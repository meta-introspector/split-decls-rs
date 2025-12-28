macro_rules! deps {
    () => {
        CtorKind!();
    };
}

macro_rules! ItemKind {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ItemKind { Fn , Static , Const , Ctor (CtorKind) , }
    };
}

ItemKind!();