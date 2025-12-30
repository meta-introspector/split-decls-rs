// Generated macro for Collectible (trait)
macro_rules! Depcrate_collectibleCollectible {
() => {
// Module: crate::collectible
// Provides: {"Collectible"}
// Dependencies: {}
# [doc = " [`Collectible`] defines the memory layout for the type in order to be passed to the garbage"] # [doc = " collector."] pub (super) trait Collectible { # [doc = " Returns the next [`Collectible`] pointer."] fn next_ptr (& self) -> Option < NonNull < dyn Collectible > > ; # [doc = " Sets the next [`Collectible`] pointer."] fn set_next_ptr (& self , next_ptr : Option < NonNull < dyn Collectible > >) ; }
};
}
