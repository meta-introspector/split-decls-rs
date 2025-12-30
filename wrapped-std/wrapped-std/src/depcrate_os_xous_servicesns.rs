// Generated macro for ns (module)
macro_rules! Depcrate_os_xous_servicesns {
() => {
// Module: crate::os::xous::services
// Provides: {"ns"}
// Dependencies: {}
mod ns { const NAME_MAX_LENGTH : usize = 64 ; use crate :: os :: xous :: ffi :: { Connection , lend_mut } ; # [repr (C , align (4096))] struct ConnectRequest { data : [u8 ; 4096] , } impl ConnectRequest { pub fn new (name : & str) -> Self { let mut cr = ConnectRequest { data : [0u8 ; 4096] } ; let name_bytes = name . as_bytes () ; for (& src_byte , dest_byte) in name_bytes . iter () . zip (& mut cr . data [0 .. NAME_MAX_LENGTH]) { * dest_byte = src_byte ; } for (& src_byte , dest_byte) in (name . len () . min (NAME_MAX_LENGTH) as u32) . to_le_bytes () . iter () . zip (& mut cr . data [NAME_MAX_LENGTH ..]) { * dest_byte = src_byte ; } cr } } pub fn connect_with_name_impl (name : & str , blocking : bool) -> Option < Connection > { let mut request = ConnectRequest :: new (name) ; let opcode = if blocking { 6 } else { 7 } ; let cid = if blocking { super :: name_server () } else { super :: try_name_server () ? } ; lend_mut (cid , opcode , & mut request . data , 0 , name . len () . min (NAME_MAX_LENGTH)) . expect ("unable to perform lookup") ; let result = u32 :: from_le_bytes (request . data [0 .. 4] . try_into () . unwrap ()) ; if result == 0 { Some (u32 :: from_le_bytes (request . data [4 .. 8] . try_into () . unwrap ()) . into ()) } else { None } } pub fn connect_with_name (name : & str) -> Option < Connection > { connect_with_name_impl (name , true) } pub fn try_connect_with_name (name : & str) -> Option < Connection > { connect_with_name_impl (name , false) } }
};
}
