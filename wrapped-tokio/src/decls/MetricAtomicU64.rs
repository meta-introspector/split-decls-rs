macro_rules! MetricAtomicU64 {
    () => {
        # [doc = " `AtomicU64` that is a no-op on platforms without 64-bit atomics"] # [doc = ""] # [doc = " When used on platforms without 64-bit atomics, writes to this are no-ops."] # [doc = " The `load` method is only defined when 64-bit atomics are available."] # [derive (Debug , Default)] pub (crate) struct MetricAtomicU64 { # [cfg (target_has_atomic = "64")] value : AtomicU64 , }
    };
}

MetricAtomicU64!();