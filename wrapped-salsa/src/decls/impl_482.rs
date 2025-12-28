macro_rules! deps {
    () => {
        QueryEdgeKind!();
        DatabaseKeyIndex!();
        QueryEdge!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl QueryEdge { # [doc = " Create an input query edge with the given index."] pub fn input (key : DatabaseKeyIndex) -> QueryEdge { Self { key } } # [doc = " Create an output query edge with the given index."] pub fn output (key : DatabaseKeyIndex) -> QueryEdge { let ingredient_index = key . ingredient_index () . with_tag (true) ; Self { key : DatabaseKeyIndex :: new (ingredient_index , key . key_index ()) , } } # [doc = " Return the key of this query edge."] pub fn key (self) -> DatabaseKeyIndex { DatabaseKeyIndex :: new (self . key . ingredient_index () . with_tag (false) , self . key . key_index () ,) } # [doc = " Returns the kind of this query edge."] pub fn kind (self) -> QueryEdgeKind { if self . key . ingredient_index () . tag () { QueryEdgeKind :: Output (self . key ()) } else { QueryEdgeKind :: Input (self . key ()) } } }
    };
}

impl_482!();