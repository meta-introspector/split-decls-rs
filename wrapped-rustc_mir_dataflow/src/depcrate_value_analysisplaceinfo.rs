// Generated macro for PlaceInfo (struct)
macro_rules! Depcrate_value_analysisPlaceInfo {
() => {
// Module: crate::value_analysis
// Provides: {"PlaceInfo"}
// Dependencies: {}
# [doc = " This is the information tracked for every [`PlaceIndex`] and is stored by [`Map`]."] # [doc = ""] # [doc = " Together, `first_child` and `next_sibling` form an intrusive linked list, which is used to"] # [doc = " model a tree structure (a replacement for a member like `children: Vec<PlaceIndex>`)."] # [derive (Debug)] struct PlaceInfo < 'tcx > { # [doc = " Type of the referenced place."] ty : Ty < 'tcx > , # [doc = " We store a [`ValueIndex`] if and only if the placed is tracked by the analysis."] value_index : Option < ValueIndex > , # [doc = " The projection used to go from parent to this node (only None for root)."] proj_elem : Option < TrackElem > , # [doc = " The leftmost child."] first_child : Option < PlaceIndex > , # [doc = " Index of the sibling to the right of this node."] next_sibling : Option < PlaceIndex > , }
};
}
