// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (not (windows))] { compile_error ! ("OS not supported. if your application is multi-platform, use `[target.'cfg(windows)'.dependencies] winreg = \"...\"`") ; } else { pub use crate :: reg_key :: { EnumKeys , EnumValues , RegKey , HKEY } ; pub use crate :: reg_key_metadata :: RegKeyMetadata ; pub use crate :: reg_value :: RegValue ; mod common ; # [cfg (feature = "serialization-serde")] pub mod decoder ; # [cfg (feature = "serialization-serde")] pub mod encoder ; pub mod enums ; pub mod reg_key ; pub mod reg_key_metadata ; pub mod reg_value ; # [cfg (feature = "transactions")] pub mod transaction ; pub mod types ; } }
};
}
