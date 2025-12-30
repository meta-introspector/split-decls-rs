// Generated macro for wrapped (module)
macro_rules! Depcratewrapped {
() => {
// Module: crate
// Provides: {"wrapped"}
// Dependencies: {}
# [cfg (any (feature = "test-nom" , feature = "test-hashbrown"))] mod wrapped { # [cfg (feature = "test-nom")] pub mod nom_wrapper { pub use nom :: * ; } # [cfg (feature = "test-hashbrown")] pub mod hashbrown_wrapper { pub use hashbrown :: * ; } }
};
}
