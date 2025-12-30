// Generated macro for SenderFlavor (enum)
macro_rules! Depcrate_sync_mpmcSenderFlavor {
() => {
// Module: crate::sync::mpmc
// Provides: {"SenderFlavor"}
// Dependencies: {}
# [doc = " Sender flavors."] enum SenderFlavor < T > { # [doc = " Bounded channel based on a preallocated array."] Array (counter :: Sender < array :: Channel < T > >) , # [doc = " Unbounded channel implemented as a linked list."] List (counter :: Sender < list :: Channel < T > >) , # [doc = " Zero-capacity channel."] Zero (counter :: Sender < zero :: Channel < T > >) , }
};
}
