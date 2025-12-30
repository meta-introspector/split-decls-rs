// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: thread ; use tokio_sync :: oneshot ; use super :: * ; fn block_on < F : Future > (f : F) -> F :: Output { tokio_executor :: enter () . expect ("block_on enter") . block_on (f) } # [test] fn want_ready () { let (mut gv , mut tk) = new () ; tk . want () ; block_on (gv . want ()) . unwrap () ; } # [test] fn want_notify_0 () { let (mut gv , mut tk) = new () ; let (tx , rx) = oneshot :: channel () ; thread :: spawn (move | | { tk . want () ; block_on (rx) . expect ("rx") ; }) ; block_on (gv . want ()) . expect ("want") ; assert ! (gv . is_wanting () , "still wanting after poll_want success") ; assert ! (gv . give () , "give is true when wanting") ; assert ! (! gv . is_wanting () , "no longer wanting after give") ; assert ! (! gv . is_canceled () , "give doesn't cancel") ; assert ! (! gv . give () , "give is false if not wanting") ; tx . send (()) . expect ("tx") ; } # [test] fn cancel () { let (mut gv , mut tk) = new () ; assert ! (! gv . is_canceled ()) ; tk . cancel () ; assert ! (gv . is_canceled ()) ; block_on (gv . want ()) . unwrap_err () ; let (mut gv , tk) = new () ; assert ! (! gv . is_canceled ()) ; drop (tk) ; assert ! (gv . is_canceled ()) ; block_on (gv . want ()) . unwrap_err () ; let (mut gv , tk) = new () ; thread :: spawn (move | | { let _tk = tk ; }) ; block_on (gv . want ()) . unwrap_err () ; } }
};
}
