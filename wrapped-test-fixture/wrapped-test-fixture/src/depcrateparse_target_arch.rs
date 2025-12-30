// Generated macro for parse_target_arch (function)
macro_rules! Depcrateparse_target_arch {
() => {
// Module: crate
// Provides: {"parse_target_arch"}
// Dependencies: {}
fn parse_target_arch (arch : & str) -> base_db :: target :: Arch { use base_db :: target :: Arch :: * ; match arch { "wasm32" => Wasm32 , "wasm64" => Wasm64 , _ => Other , } }
};
}
