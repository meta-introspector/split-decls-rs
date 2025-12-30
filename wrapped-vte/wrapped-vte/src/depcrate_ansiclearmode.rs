// Generated macro for ClearMode (enum)
macro_rules! Depcrate_ansiClearMode {
() => {
// Module: crate::ansi
// Provides: {"ClearMode"}
// Dependencies: {}
# [doc = " Mode for clearing terminal."] # [doc = ""] # [doc = " Relative to cursor."] # [derive (Debug)] pub enum ClearMode { # [doc = " Clear below cursor."] Below , # [doc = " Clear above cursor."] Above , # [doc = " Clear entire terminal."] All , # [doc = " Clear 'saved' lines (scrollback)."] Saved , }
};
}
