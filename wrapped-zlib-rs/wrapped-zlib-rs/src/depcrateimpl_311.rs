// Generated macro for impl_311 (impl)
macro_rules! Depcrateimpl_311 {
() => {
// Module: crate
// Provides: {"impl_311"}
// Dependencies: {}
impl ReturnCode { const fn error_message_str (self) -> & 'static str { match self { ReturnCode :: Ok => "\0" , ReturnCode :: StreamEnd => "stream end\0" , ReturnCode :: NeedDict => "need dictionary\0" , ReturnCode :: ErrNo => "file error\0" , ReturnCode :: StreamError => "stream error\0" , ReturnCode :: DataError => "data error\0" , ReturnCode :: MemError => "insufficient memory\0" , ReturnCode :: BufError => "buffer error\0" , ReturnCode :: VersionError => "incompatible version\0" , } } pub const fn error_message (self) -> * const core :: ffi :: c_char { let msg = self . error_message_str () ; msg . as_ptr () . cast :: < core :: ffi :: c_char > () } pub const fn try_from_c_int (err : core :: ffi :: c_int) -> Option < Self > { match err { 0 => Some (Self :: Ok) , 1 => Some (Self :: StreamEnd) , 2 => Some (Self :: NeedDict) , - 1 => Some (Self :: ErrNo) , - 2 => Some (Self :: StreamError) , - 3 => Some (Self :: DataError) , - 4 => Some (Self :: MemError) , - 5 => Some (Self :: BufError) , - 6 => Some (Self :: VersionError) , _ => None , } } }
};
}
