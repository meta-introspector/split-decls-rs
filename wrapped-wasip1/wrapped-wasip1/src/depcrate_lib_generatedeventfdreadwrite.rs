// Generated macro for EventFdReadwrite (struct)
macro_rules! Depcrate_lib_generatedEventFdReadwrite {
() => {
// Module: crate::lib_generated
// Provides: {"EventFdReadwrite"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone , Debug)] pub struct EventFdReadwrite { # [doc = " The number of bytes available for reading or writing."] pub nbytes : Filesize , # [doc = " The state of the file descriptor."] pub flags : Eventrwflags , }
};
}
