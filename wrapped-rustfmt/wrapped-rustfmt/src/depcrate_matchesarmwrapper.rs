// Generated macro for ArmWrapper (struct)
macro_rules! Depcrate_matchesArmWrapper {
() => {
// Module: crate::matches
// Provides: {"ArmWrapper"}
// Dependencies: {}
# [doc = " A simple wrapper type against `ast::Arm`. Used inside `write_list()`."] struct ArmWrapper < 'a > { arm : & 'a ast :: Arm , # [doc = " `true` if the arm is the last one in match expression. Used to decide on whether we should"] # [doc = " add trailing comma to the match arm when `config.trailing_comma() == Never`."] is_last : bool , # [doc = " Holds a byte position of `|` at the beginning of the arm pattern, if available."] beginning_vert : Option < BytePos > , }
};
}
