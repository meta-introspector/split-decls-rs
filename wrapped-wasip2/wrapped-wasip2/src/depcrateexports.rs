// Generated macro for exports (module)
macro_rules! Depcrateexports {
() => {
// Module: crate
// Provides: {"exports"}
// Dependencies: {}
pub mod exports { # [doc (hidden)] pub mod wasi { pub use crate :: command :: exports :: wasi :: * ; pub use crate :: proxy :: exports :: wasi :: * ; } pub use crate :: command :: exports :: wasi :: cli ; pub use crate :: proxy :: exports :: wasi :: http ; }
};
}
