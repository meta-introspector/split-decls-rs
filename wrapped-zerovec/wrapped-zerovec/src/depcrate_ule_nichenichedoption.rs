// Generated macro for NichedOption (struct)
macro_rules! Depcrate_ule_nicheNichedOption {
() => {
// Module: crate::ule::niche
// Provides: {"NichedOption"}
// Dependencies: {}
# [doc = " Optional type which uses [`NichedOptionULE<U,N>`] as ULE type."] # [doc = ""] # [doc = " The implementors guarantee that `N == core::mem::size_of::<Self>()`"] # [doc = " `#[repr(transparent)]` guarantees that the layout is same as [`Option<U>`]"] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [repr (transparent)] # [allow (clippy :: exhaustive_structs)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct NichedOption < U , const N : usize > (pub Option < U >) ;
};
}
