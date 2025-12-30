// Generated macro for Reader (struct)
macro_rules! Depcrate_msgs_codecReader {
() => {
// Module: crate::msgs::codec
// Provides: {"Reader"}
// Dependencies: {}
# [doc = " Wrapper over a slice of bytes that allows reading chunks from"] # [doc = " with the current position state held using a cursor."] # [doc = ""] # [doc = " A new reader for a sub section of the buffer can be created"] # [doc = " using the `sub` function or a section of a certain length can"] # [doc = " be obtained using the `take` function"] pub struct Reader < 'a > { # [doc = " The underlying buffer storing the readers content"] buffer : & 'a [u8] , # [doc = " Stores the current reading position for the buffer"] cursor : usize , }
};
}
