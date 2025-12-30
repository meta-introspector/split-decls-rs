// Generated macro for OptionalActions (enum)
macro_rules! Depcrate_termios_typesOptionalActions {
() => {
// Module: crate::termios::types
// Provides: {"OptionalActions"}
// Dependencies: {}
# [doc = " `TCSA*` values for use with [`tcsetattr`]."] # [doc = ""] # [doc = " [`tcsetattr`]: crate::termios::tcsetattr"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (u32)] pub enum OptionalActions { # [doc = " `TCSANOW`—Make the change immediately."] # [doc (alias = "TCSANOW")] Now = c :: TCSANOW as u32 , # [doc = " `TCSADRAIN`—Make the change after all output has been transmitted."] # [doc (alias = "TCSADRAIN")] Drain = c :: TCSADRAIN as u32 , # [doc = " `TCSAFLUSH`—Discard any pending input and then make the change"] # [doc = " after all output has been transmitted."] # [doc (alias = "TCSAFLUSH")] Flush = c :: TCSAFLUSH as u32 , }
};
}
