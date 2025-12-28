macro_rules! deps {
    () => {
        Edge!();
        FxIndexSet!();
    };
}

macro_rules! TransitiveRelationBuilder {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct TransitiveRelationBuilder < T > { elements : FxIndexSet < T > , edges : FxHashSet < Edge > , }
    };
}

TransitiveRelationBuilder!()