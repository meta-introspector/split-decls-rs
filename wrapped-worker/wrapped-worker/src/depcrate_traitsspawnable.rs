// Generated macro for Spawnable (trait)
macro_rules! Depcrate_traitsSpawnable {
() => {
// Module: crate::traits
// Provides: {"Spawnable"}
// Dependencies: {}
# [doc = " A Worker that can be spawned by a spawner."] pub trait Spawnable { # [doc = " Spawner Type."] type Spawner ; # [doc = " Creates a spawner."] fn spawner () -> Self :: Spawner ; }
};
}
