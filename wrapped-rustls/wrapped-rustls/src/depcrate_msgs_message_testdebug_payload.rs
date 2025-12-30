// Generated macro for debug_payload (function)
macro_rules! Depcrate_msgs_message_testdebug_payload {
() => {
// Module: crate::msgs::message_test
// Provides: {"debug_payload"}
// Dependencies: {}
# [test] fn debug_payload () { assert_eq ! ("01020304" , format ! ("{:?}" , Payload :: new (vec ! [1 , 2 , 3 , 4]))) ; assert_eq ! ("01020304" , format ! ("{:?}" , PayloadU8 ::< NonEmpty >:: new (vec ! [1 , 2 , 3 , 4]))) ; assert_eq ! ("01020304" , format ! ("{:?}" , PayloadU16 ::< MaybeEmpty >:: new (vec ! [1 , 2 , 3 , 4]))) ; assert_eq ! ("01020304" , format ! ("{:?}" , PayloadU24 ::<'static , NonEmpty >:: from (Payload :: new (vec ! [1 , 2 , 3 , 4])))) ; }
};
}
