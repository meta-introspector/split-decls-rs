// Generated macro for impl_159 (impl)
macro_rules! Depcrate_intrinsicimpl_159 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_159"}
// Dependencies: {}
impl fmt :: Display for UnsafetyComment { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Custom (s) => s . fmt (f) , Self :: Neon => write ! (f , "Neon instrinsic unsafe") , Self :: Uninitialized => write ! (f , "This creates an uninitialized value, and may be unsound (like \
                [`core::mem::uninitialized`]).") , Self :: PointerOffset (gov) => write ! (f , "[`pointer::offset`](pointer#method.offset) safety constraints must \
                be met for the address calculation for each active element{gov}.") , Self :: PointerOffsetVnum (gov) => write ! (f , "[`pointer::offset`](pointer#method.offset) safety constraints must \
                be met for the address calculation for each active element{gov}. \
                In particular, note that `vnum` is scaled by the vector \
                length, `VL`, which is not known at compile time.") , Self :: Dereference (gov) => write ! (f , "This dereferences and accesses the calculated address for each \
                active element{gov}.") , Self :: NonTemporal => write ! (f , "Non-temporal accesses have special memory ordering rules, and \
                [explicit barriers may be required for some applications]\
                (https://developer.arm.com/documentation/den0024/a/Memory-Ordering/Barriers/Non-temporal-load-and-store-pair?lang=en).") , Self :: NoProvenance (arg) => write ! (f , "Addresses passed in `{arg}` lack provenance, so this is similar to using a \
                `usize as ptr` cast (or [`core::ptr::from_exposed_addr`]) on each lane before \
                using it.") , Self :: UnpredictableOnFault => write ! (f , "Result lanes corresponding to inactive FFR lanes (either before or as a result \
                of this intrinsic) have \"CONSTRAINED UNPREDICTABLE\" values, irrespective of \
                predication. Refer to architectural documentation for details.") , } } }
};
}
