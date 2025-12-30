// Generated macro for array (function)
macro_rules! Depcrate_bound_checksarray {
() => {
// Module: crate::bound_checks
// Provides: {"array"}
// Dependencies: {}
# [test] fn array () { let arr = NSArray :: < NSObject > :: new () ; assert_throws ("index 0 beyond bounds" , | | arr . objectAtIndex (0)) ; assert_throws ("index 0 beyond bounds" , | | arr . objectAtIndexedSubscript (0)) ; let arr = NSArray :: from_retained_slice (& [NSObject :: new () , NSObject :: new ()]) ; assert_throws ("index 100 beyond bounds [0 .. 1]" , | | { arr . objectAtIndex (100) }) ; let arr = NSMutableArray :: < NSObject > :: new () ; assert_throws ("index 100 beyond bounds" , | | { arr . insertObject_atIndex (& NSObject :: new () , 100) }) ; assert_throws ("range {0, 1} extends beyond bounds" , | | { arr . removeObjectAtIndex (0) }) ; assert_throws ("index 100 beyond bounds" , | | { arr . replaceObjectAtIndex_withObject (100 , & NSObject :: new ()) }) ; }
};
}
