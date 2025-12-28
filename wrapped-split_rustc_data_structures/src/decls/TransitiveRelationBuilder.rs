macro_rules! deps {
    () => {
        FxIndexSet!();
        Edge!();
    };
}

macro_rules! TransitiveRelationBuilder {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct TransitiveRelationBuilder < T > { elements : FxIndexSet < T > , edges : FxHashSet < Edge > , }
    };
}

TransitiveRelationBuilder!();