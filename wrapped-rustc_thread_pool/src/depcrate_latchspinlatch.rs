// Generated macro for SpinLatch (struct)
macro_rules! Depcrate_latchSpinLatch {
() => {
// Module: crate::latch
// Provides: {"SpinLatch"}
// Dependencies: {}
# [doc = " Spin latches are the simplest, most efficient kind, but they do"] # [doc = " not support a `wait()` operation. They just have a boolean flag"] # [doc = " that becomes true when `set()` is called."] pub (super) struct SpinLatch < 'r > { core_latch : CoreLatch , registry : & 'r Arc < Registry > , target_worker_index : usize , cross : bool , }
};
}
