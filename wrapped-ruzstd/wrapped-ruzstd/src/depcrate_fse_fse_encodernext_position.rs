// Generated macro for next_position (function)
macro_rules! Depcrate_fse_fse_encodernext_position {
() => {
// Module: crate::fse::fse_encoder
// Provides: {"next_position"}
// Dependencies: {}
# [doc = " Calculate the position of the next entry of the table given the current"] # [doc = " position and size of the table."] fn next_position (mut p : usize , table_size : usize) -> usize { p += (table_size >> 1) + (table_size >> 3) + 3 ; p &= table_size - 1 ; p }
};
}
