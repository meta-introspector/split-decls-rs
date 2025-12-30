// Generated macro for DatabaseKeyIndex (struct)
macro_rules! Depcrate_keyDatabaseKeyIndex {
() => {
// Module: crate::key
// Provides: {"DatabaseKeyIndex"}
// Dependencies: {}
# [doc = " An integer that uniquely identifies a particular query instance within the"] # [doc = " database. Used to track input and output dependencies between queries. Fully"] # [doc = " ordered and equatable but those orderings are arbitrary, and meant to be used"] # [doc = " only for inserting into maps and the like."] # [derive (Copy , Clone , PartialEq , Eq , Hash)] pub struct DatabaseKeyIndex { key_index : Id , ingredient_index : IngredientIndex , }
};
}
