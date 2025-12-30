// Generated macro for TestNonBlockIo (struct)
macro_rules! DepcrateTestNonBlockIo {
() => {
// Module: crate
// Provides: {"TestNonBlockIo"}
// Dependencies: {}
# [doc = " An object that impls `io::Read` and `io::Write` for testing."] # [doc = ""] # [doc = " The `reads` and `writes` fields set the behaviour of these trait"] # [doc = " implementations.  They return the `WouldBlock` error if not otherwise"] # [doc = " configured -- `TestNonBlockIo::default()` does this permanently."] # [doc = ""] # [doc = " This object panics on drop if the configured expected reads/writes"] # [doc = " didn't take place."] # [derive (Debug , Default)] pub struct TestNonBlockIo { # [doc = " Each `write()` call is satisfied by inspecting this field."] # [doc = ""] # [doc = " If it is empty, `WouldBlock` is returned.  Otherwise the write is"] # [doc = " satisfied by popping a value and returning it (reduced by the size"] # [doc = " of the write buffer, if needed)."] pub writes : Vec < usize > , # [doc = " Each `read()` call is satisfied by inspecting this field."] # [doc = ""] # [doc = " If it is empty, `WouldBlock` is returned.  Otherwise the read is"] # [doc = " satisfied by popping a value and copying it into the output"] # [doc = " buffer.  Each value must be no longer than the buffer for that"] # [doc = " call."] pub reads : Vec < Vec < u8 > > , }
};
}
