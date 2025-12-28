macro_rules! deps {
    () => {
        TransitiveRelationBuilder!();
        Frozen!();
        TransitiveRelation!();
    };
}

macro_rules! impl_617 {
    () => {
        deps!();
        impl < T > Deref for TransitiveRelation < T > { type Target = Frozen < TransitiveRelationBuilder < T > > ; fn deref (& self) -> & Self :: Target { & self . builder } }
    };
}

impl_617!();