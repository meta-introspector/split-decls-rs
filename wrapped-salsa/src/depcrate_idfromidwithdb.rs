// Generated macro for FromIdWithDb (trait)
macro_rules! Depcrate_idFromIdWithDb {
() => {
// Module: crate::id
// Provides: {"FromIdWithDb"}
// Dependencies: {}
# [doc = " Enums cannot use [`FromId`] because they need access to the DB to tell the `TypeId` of the variant,"] # [doc = " so they use this trait instead, that has a blanket implementation for `FromId`."] pub trait FromIdWithDb { fn from_id (id : Id , zalsa : & Zalsa) -> Self ; }
};
}
