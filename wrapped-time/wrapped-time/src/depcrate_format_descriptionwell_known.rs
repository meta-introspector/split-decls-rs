// Generated macro for well_known (module)
macro_rules! Depcrate_format_descriptionwell_known {
() => {
// Module: crate::format_description
// Provides: {"well_known"}
// Dependencies: {}
# [doc = " Well-known formats, typically standards."] pub mod well_known { pub mod iso8601 ; mod rfc2822 ; mod rfc3339 ; # [doc (inline)] pub use iso8601 :: Iso8601 ; pub use rfc2822 :: Rfc2822 ; pub use rfc3339 :: Rfc3339 ; }
};
}
