// Generated macro for tests (module)
macro_rules! Depcrate_buildtests {
() => {
// Module: crate::build
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use core :: sync :: atomic :: { AtomicUsize , Ordering } ; use super :: * ; # [test] fn test_build_array () { let array = build_array (| i | i * 2) ; assert_eq ! (array , [0 , 2 , 4]) ; } # [test] fn test_build_array_option () { let array = build_array_option (| i | Some (i * 2)) ; assert_eq ! (array , Some ([0 , 2 , 4])) ; let none : Option < [_ ; 10] > = build_array_option (| i | if i == 5 { None } else { Some (()) }) ; assert_eq ! (none , None) ; } # [test] fn test_build_array_result () { let array = build_array_result (| i | Ok :: < usize , () > (i * 2)) ; assert_eq ! (array , Ok ([0 , 2 , 4])) ; let err : Result < [_ ; 10] , _ > = build_array_result (| i | if i == 5 { Err (()) } else { Ok (()) }) ; assert_eq ! (err , Err (())) ; } struct IncrementOnDrop < 'a > (& 'a AtomicUsize) ; impl Drop for IncrementOnDrop < '_ > { fn drop (& mut self) { self . 0 . fetch_add (1 , Ordering :: Relaxed) ; } } # [test] fn result_doesnt_leak_on_err () { let drop_counter = 0 . into () ; let _ : Result < [_ ; 5] , _ > = build_array_result (| i | { if i == 3 { Err (()) } else { Ok (IncrementOnDrop (& drop_counter)) } }) ; assert_eq ! (drop_counter . load (Ordering :: Relaxed) , 3) ; } # [test] fn option_doesnt_leak_on_err () { let drop_counter = 0 . into () ; let _ : Option < [_ ; 5] > = build_array_option (| i | { if i == 3 { None } else { Some (IncrementOnDrop (& drop_counter)) } }) ; assert_eq ! (drop_counter . load (Ordering :: Relaxed) , 3) ; } }
};
}
