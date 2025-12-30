// Generated macro for capture_retained (function)
macro_rules! Depcrate_blockcapture_retained {
() => {
// Module: crate::block
// Provides: {"capture_retained"}
// Dependencies: {}
# [test] fn capture_retained () { let obj1 = NSObject :: new () ; let obj2 = NSObject :: new () ; let closure = move | | Bool :: new (obj1 == obj2) ; struct Enc ; unsafe impl ManualBlockEncoding for Enc { type Arguments = () ; type Return = Bool ; const ENCODING_CSTR : & 'static CStr = { match (Bool :: ENCODING , cfg ! (target_pointer_width = "64")) { (Encoding :: Char , true) => c"c8@?0" , (Encoding :: UChar , true) => c"C8@?0" , (Encoding :: Bool , true) => c"b8@?0" , (Encoding :: Char , false) => c"c4@?0" , (Encoding :: UChar , false) => c"C4@?0" , (Encoding :: Bool , false) => c"b4@?0" , _ => panic ! ("invalid Bool encoding") , } } ; } for stack_block in [StackBlock :: new (closure . clone ()) , StackBlock :: with_encoding :: < Enc > (closure) ,] { let rc_block = stack_block . copy () ; assert ! (rc_block . call (()) . is_false ()) ; } }
};
}
