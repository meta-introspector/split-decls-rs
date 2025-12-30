// Generated macro for Reserve (type)
macro_rules! Depcrate_hash_setReserve {
() => {
// Module: crate::hash_set
// Provides: {"Reserve"}
// Dependencies: {}
# [doc = " [`Reserve`] keeps the capacity of the associated [`HashSet`] higher than a certain level."] # [doc = ""] # [doc = " The [`HashSet`] does not shrink the capacity below the reserved capacity."] pub type Reserve < 'h , K , H = RandomState > = super :: hash_map :: Reserve < 'h , K , () , H > ;
};
}
