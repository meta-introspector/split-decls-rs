// Generated macro for Tree (enum)
macro_rules! Depcrate_treeTree {
() => {
// Module: crate::tree
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " A node in the log tree, consisting of either a [`Span`] or an [`Event`]."] # [doc = ""] # [doc = " The inner types can be extracted through a `match` statement. Alternatively,"] # [doc = " the [`event`] and [`span`] methods provide a more ergonomic way to access the"] # [doc = " inner types in unit tests when combined with the [`capture`] function."] # [doc = ""] # [doc = " [`event`]: Tree::event"] # [doc = " [`span`]: Tree::span"] # [doc = " [`capture`]: crate::runtime::capture"] # [derive (Clone , Debug)] # [cfg_attr (feature = "serde" , derive (Serialize))] # [allow (clippy :: large_enum_variant)] pub enum Tree { # [doc = " An [`Event`] leaf node."] Event (Event) , # [doc = " A [`Span`] inner node."] Span (Span) , }
};
}
