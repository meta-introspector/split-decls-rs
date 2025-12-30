// Generated macro for prelude (module)
macro_rules! Depcrateprelude {
() => {
// Module: crate
// Provides: {"prelude"}
// Dependencies: {}
# [doc = " Easier access to common traits"] pub mod prelude { pub use crate :: IntoData ; # [cfg (feature = "json")] pub use crate :: IntoJson ; pub use crate :: ToDebug ; }
};
}
