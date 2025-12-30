// Generated macro for other_39 (other)
macro_rules! Depcrate_util_macro_utilother_39 {
() => {
// Module: crate::util::macro_util
// Provides: {"other_39"}
// Dependencies: {}
# [doc = " A type whose size is equal to `max(align_of::<T>(), align_of::<U>())`."] # [repr (C)] pub union MaxAlignsOf < T , U > { _t : ManuallyDrop < AlignOf < T > > , _u : ManuallyDrop < AlignOf < U > > , }
};
}
