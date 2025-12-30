// Generated macro for construct_all_types (function)
macro_rules! Depcrate_msgs_message_testconstruct_all_types {
() => {
// Module: crate::msgs::message_test
// Provides: {"construct_all_types"}
// Dependencies: {}
# [test] fn construct_all_types () { let samples = [& b"\x14\x03\x04\x00\x01\x01" [..] , & b"\x15\x03\x04\x00\x02\x01\x16" [..] , & b"\x16\x03\x04\x00\x05\x18\x00\x00\x01\x00" [..] , & b"\x17\x03\x04\x00\x04\x11\x22\x33\x44" [..] , & b"\x18\x03\x04\x00\x04\x11\x22\x33\x44" [..] ,] ; for & bytes in samples . iter () { let m = PlainMessage :: read (& mut Reader :: init (bytes)) . unwrap () ; println ! ("m = {m:?}") ; let m = Message :: try_from (m) ; println ! ("m' = {m:?}") ; } }
};
}
