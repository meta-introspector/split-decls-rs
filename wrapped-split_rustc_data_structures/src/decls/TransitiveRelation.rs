macro_rules! deps {
    () => {
        Frozen!();
        TransitiveRelationBuilder!();
    };
}

macro_rules! TransitiveRelation {
    () => {
        deps!();
        # [derive (Debug)] pub struct TransitiveRelation < T > { builder : Frozen < TransitiveRelationBuilder < T > > , closure : Frozen < BitMatrix < usize , usize > > , }
    };
}

TransitiveRelation!()