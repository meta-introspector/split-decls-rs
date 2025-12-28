macro_rules! deps {
    () => {
        Interner!();
        DefId!();
    };
}

macro_rules! SpecificDefId {
    () => {
        deps!();
        pub trait SpecificDefId < I : Interner > : DefId < I > + Into < I :: DefId > + TryFrom < I :: DefId , Error : std :: fmt :: Debug > { }
    };
}

SpecificDefId!();