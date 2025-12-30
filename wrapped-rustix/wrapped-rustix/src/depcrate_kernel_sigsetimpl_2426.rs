// Generated macro for impl_2426 (impl)
macro_rules! Depcrate_kernel_sigsetimpl_2426 {
() => {
// Module: crate::kernel_sigset
// Provides: {"impl_2426"}
// Dependencies: {}
impl KernelSigSet { # [doc = " Create a new empty `KernelSigSet`."] pub const fn empty () -> Self { const fn zeros < const N : usize > () -> [c :: c_ulong ; N] { [0 ; N] } Self (kernel_sigset_t { sig : zeros () }) } # [doc = " Create a new `KernelSigSet` with all signals set."] # [doc = ""] # [doc = " This includes signals which are typically reserved for libc."] pub const fn all () -> Self { const fn ones < const N : usize > () -> [c :: c_ulong ; N] { [! 0 ; N] } Self (kernel_sigset_t { sig : ones () }) } # [doc = " Remove all signals."] pub fn clear (& mut self) { * self = Self (kernel_sigset_t { sig : Default :: default () , }) ; } # [doc = " Insert a signal."] pub fn insert (& mut self , sig : Signal) { let sigs_per_elt = core :: mem :: size_of_val (& self . 0 . sig [0]) * 8 ; let raw = (sig . as_raw () . wrapping_sub (1)) as usize ; self . 0 . sig [raw / sigs_per_elt] |= 1 << (raw % sigs_per_elt) ; } # [doc = " Insert all signals."] pub fn insert_all (& mut self) { self . 0 . sig . fill (! 0) ; } # [doc = " Remove a signal."] pub fn remove (& mut self , sig : Signal) { let sigs_per_elt = core :: mem :: size_of_val (& self . 0 . sig [0]) * 8 ; let raw = (sig . as_raw () . wrapping_sub (1)) as usize ; self . 0 . sig [raw / sigs_per_elt] &= ! (1 << (raw % sigs_per_elt)) ; } # [doc = " Test whether a given signal is present."] pub fn contains (& self , sig : Signal) -> bool { let sigs_per_elt = core :: mem :: size_of_val (& self . 0 . sig [0]) * 8 ; let raw = (sig . as_raw () . wrapping_sub (1)) as usize ; (self . 0 . sig [raw / sigs_per_elt] & (1 << (raw % sigs_per_elt))) != 0 } }
};
}
