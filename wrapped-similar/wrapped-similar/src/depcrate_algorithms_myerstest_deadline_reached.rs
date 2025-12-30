// Generated macro for test_deadline_reached (function)
macro_rules! Depcrate_algorithms_myerstest_deadline_reached {
() => {
// Module: crate::algorithms::myers
// Provides: {"test_deadline_reached"}
// Dependencies: {}
# [test] fn test_deadline_reached () { use std :: ops :: Index ; use std :: time :: Duration ; let a = (0 .. 100) . collect :: < Vec < _ > > () ; let mut b = (0 .. 100) . collect :: < Vec < _ > > () ; b [10] = 99 ; b [50] = 99 ; b [25] = 99 ; struct SlowIndex < 'a > (& 'a [usize]) ; impl Index < usize > for SlowIndex < '_ > { type Output = usize ; fn index (& self , index : usize) -> & Self :: Output { std :: thread :: sleep (Duration :: from_millis (1)) ; & self . 0 [index] } } let slow_a = SlowIndex (& a) ; let slow_b = SlowIndex (& b) ; let mut d = crate :: algorithms :: Replace :: new (crate :: algorithms :: Capture :: new ()) ; diff_deadline (& mut d , & slow_a , 0 .. a . len () , & slow_b , 0 .. b . len () , Some (Instant :: now () + Duration :: from_millis (50)) ,) . unwrap () ; insta :: assert_debug_snapshot ! (d . into_inner () . ops ()) ; }
};
}
