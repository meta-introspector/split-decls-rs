// Generated macro for ReceiverFlavor (enum)
macro_rules! Depcrate_sync_mpmcReceiverFlavor {
() => {
// Module: crate::sync::mpmc
// Provides: {"ReceiverFlavor"}
// Dependencies: {}
# [doc = " Receiver flavors."] enum ReceiverFlavor < T > { # [doc = " Bounded channel based on a preallocated array."] Array (counter :: Receiver < array :: Channel < T > >) , # [doc = " Unbounded channel implemented as a linked list."] List (counter :: Receiver < list :: Channel < T > >) , # [doc = " Zero-capacity channel."] Zero (counter :: Receiver < zero :: Channel < T > >) , }
};
}
