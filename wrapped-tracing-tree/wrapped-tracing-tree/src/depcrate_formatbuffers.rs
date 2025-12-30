// Generated macro for Buffers (struct)
macro_rules! Depcrate_formatBuffers {
() => {
// Module: crate::format
// Provides: {"Buffers"}
// Dependencies: {}
# [derive (Debug)] pub struct Buffers { pub current_buf : String , pub indent_buf : String , # [doc = " The last seen span of this layer"] # [doc = ""] # [doc = " This serves to serialize spans as two events can be generated in different spans"] # [doc = " without the spans entering and exiting beforehand. This happens for multithreaded code"] # [doc = " and instrumented futures"] pub current_span : Option < span :: Id > , }
};
}
