// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "std"))] mod tests { use super :: * ; # [test] fn before_epoch () { let t = UNIX_EPOCH - Duration :: new (0 , 1) ; let tai64n = Tai64N :: from_system_time (& t) ; let t1 = tai64n . to_system_time () ; assert_eq ! (t , t1) ; let t = UNIX_EPOCH - Duration :: new (488294802189 , 999999999) ; let tai64n = Tai64N :: from_system_time (& t) ; let t1 = tai64n . to_system_time () ; assert_eq ! (t , t1) ; let t = UNIX_EPOCH - Duration :: new (73234 , 68416841) ; let tai64n = Tai64N :: from_system_time (& t) ; let t1 = tai64n . to_system_time () ; assert_eq ! (t , t1) ; } }
};
}
