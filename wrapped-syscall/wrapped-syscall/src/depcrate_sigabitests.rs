// Generated macro for tests (module)
macro_rules! Depcrate_sigabitests {
() => {
// Module: crate::sigabi
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: sync :: { atomic :: { AtomicU64 , Ordering } , Arc , } ; # [cfg (not (loom))] use std :: { sync :: Mutex , thread } ; # [cfg (not (loom))] fn model (f : impl FnOnce ()) { f () } # [cfg (loom)] use loom :: { model , sync :: Mutex , thread } ; use crate :: { RawAction , SigProcControl , Sigcontrol } ; struct FakeThread { ctl : Sigcontrol , pctl : SigProcControl , ctxt : Mutex < () > , } impl Default for FakeThread { fn default () -> Self { Self { ctl : Sigcontrol :: default () , pctl : SigProcControl { pending : AtomicU64 :: new (0) , actions : core :: array :: from_fn (| _ | RawAction :: default ()) , sender_infos : Default :: default () , } , ctxt : Default :: default () , } } } # [test] fn singlethread_mask () { model (| | { let fake_thread = Arc :: new (FakeThread :: default ()) ; let thread = { let fake_thread = Arc :: clone (& fake_thread) ; thread :: spawn (move | | { fake_thread . ctl . set_allowset (! 0) ; { let _g = fake_thread . ctxt . lock () ; if fake_thread . ctl . currently_pending_unblocked (& fake_thread . pctl) == 0 { drop (_g) ; thread :: park () ; } } }) } ; for sig in 1 ..= 64 { let _g = fake_thread . ctxt . lock () ; let idx = sig - 1 ; let bit = 1 << (idx % 32) ; fake_thread . ctl . word [idx / 32] . fetch_or (bit , Ordering :: Relaxed) ; let w = fake_thread . ctl . word [idx / 32] . load (Ordering :: Relaxed) ; if w & (w >> 32) != 0 { thread . thread () . unpark () ; } } thread . join () . unwrap () ; }) ; } }
};
}
