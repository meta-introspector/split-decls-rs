// Generated macro for LinkedEntry (struct)
macro_rules! Depcrate_linked_listLinkedEntry {
() => {
// Module: crate::linked_list
// Provides: {"LinkedEntry"}
// Dependencies: {}
# [doc = " [`LinkedEntry`] stores an instance of `T` and a link to the next entry."] pub struct LinkedEntry < T > { # [doc = " `instance` is always `Some` unless [`Self::take_inner`] is called."] instance : Option < T > , # [doc = " `next` points to the next entry in a linked list."] next : AtomicShared < Self > , }
};
}
