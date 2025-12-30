// Generated macro for PathNode (struct)
macro_rules! Depcrate_verify_certPathNode {
() => {
// Module: crate::verify_cert
// Provides: {"PathNode"}
// Dependencies: {}
pub (crate) struct PathNode < 'a > { # [doc = " The path we're iterating."] path : & 'a PartialPath < 'a > , # [doc = " The index of the current node in the path (input for `path.get()`)."] index : usize , # [doc = " The [`Cert`] at `index`."] pub (crate) cert : & 'a Cert < 'a > , }
};
}
