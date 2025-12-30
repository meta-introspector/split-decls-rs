// Generated macro for Durability (struct)
macro_rules! Depcrate_durabilityDurability {
() => {
// Module: crate::durability
// Provides: {"Durability"}
// Dependencies: {}
# [doc = " Describes how likely a value is to change—how \"durable\" it is."] # [doc = ""] # [doc = " By default, inputs have `Durability::LOW` and interned values have"] # [doc = " `Durability::HIGH`. But inputs can be explicitly set with other"] # [doc = " durabilities."] # [doc = ""] # [doc = " We use durabilities to optimize the work of \"revalidating\" a query"] # [doc = " after some input has changed. Ordinarily, in a new revision,"] # [doc = " queries have to trace all their inputs back to the base inputs to"] # [doc = " determine if any of those inputs have changed. But if we know that"] # [doc = " the only changes were to inputs of low durability (the common"] # [doc = " case), and we know that the query only used inputs of medium"] # [doc = " durability or higher, then we can skip that enumeration."] # [doc = ""] # [doc = " Typically, one assigns low durabilities to inputs that the user is"] # [doc = " frequently editing. Medium or high durabilities are used for"] # [doc = " configuration, the source from library crates, or other things"] # [doc = " that are unlikely to be edited."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct Durability (DurabilityVal) ;
};
}
