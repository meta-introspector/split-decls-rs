// Generated macro for empty (function)
macro_rules! Depcrate_balance_p2c_testempty {
() => {
// Module: crate::balance::p2c::test
// Provides: {"empty"}
// Dependencies: {}
# [tokio :: test] async fn empty () { let empty : Vec < load :: Constant < mock :: Mock < () , & 'static str > , usize > > = vec ! [] ; let disco = ServiceList :: new (empty) ; let mut svc = mock :: Spawn :: new (Balance :: new (disco)) ; assert_pending ! (svc . poll_ready ()) ; }
};
}
