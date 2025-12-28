macro_rules! deps {
    () => {
        DatabaseKeyIndex!();
    };
}

macro_rules! QueryEdgeKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum QueryEdgeKind { Input (DatabaseKeyIndex) , Output (DatabaseKeyIndex) , }
    };
}

QueryEdgeKind!();