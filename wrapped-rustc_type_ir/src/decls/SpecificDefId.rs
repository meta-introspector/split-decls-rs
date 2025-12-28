macro_rules! deps {
    () => {
        DefId!();
        Interner!();
    };
}

macro_rules! SpecificDefId {
    () => {
        deps!();
        pub trait SpecificDefId < I : Interner > : DefId < I > + Into < I :: DefId > + TryFrom < I :: DefId , Error : std :: fmt :: Debug > { }
    };
}

SpecificDefId!()