macro_rules! deps {
    () => {
        DefId!();
        SpecificDefId!();
        Interner!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < I : Interner , T : DefId < I > + Into < I :: DefId > + TryFrom < I :: DefId , Error : std :: fmt :: Debug > > SpecificDefId < I > for T { }
    };
}

impl_76!();