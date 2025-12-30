// Generated macro for Action (enum)
macro_rules! Depcrate_termios_typesAction {
() => {
// Module: crate::termios::types
// Provides: {"Action"}
// Dependencies: {}
# [doc = " `TC*` values for use with [`tcflow`]."] # [doc = ""] # [doc = " [`tcflow`]: crate::termios::tcflow"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (u32)] pub enum Action { # [doc = " `TCOOFF`—Suspend output."] # [doc (alias = "TCOOFF")] OOff = c :: TCOOFF as u32 , # [doc = " `TCOON`—Restart suspended output."] # [doc (alias = "TCOON")] OOn = c :: TCOON as u32 , # [doc = " `TCIOFF`—Transmits a STOP byte."] # [doc (alias = "TCIOFF")] IOff = c :: TCIOFF as u32 , # [doc = " `TCION`—Transmits a START byte."] # [doc (alias = "TCION")] IOn = c :: TCION as u32 , }
};
}
