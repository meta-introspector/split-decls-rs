macro_rules! deps {
    () => {
        TransitiveRelationBuilder!();
    };
}

macro_rules! impl_619 {
    () => {
        deps!();
        impl < T : Eq + Hash > Default for TransitiveRelationBuilder < T > { fn default () -> Self { TransitiveRelationBuilder { elements : Default :: default () , edges : Default :: default () } } }
    };
}

impl_619!()