// Generated macro for impl_125 (impl)
macro_rules! Depcrate_framework_graphvizimpl_125 {
() => {
// Module: crate::framework::graphviz
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'tcx , A > dot :: Labeller < '_ > for Formatter < '_ , 'tcx , A > where A : Analysis < 'tcx > , A :: Domain : DebugWithContext < A > , { type Node = BasicBlock ; type Edge = CfgEdge ; fn graph_id (& self) -> dot :: Id < '_ > { let name = graphviz_safe_def_name (self . body . source . def_id ()) ; dot :: Id :: new (format ! ("graph_for_def_id_{name}")) . unwrap () } fn node_id (& self , n : & Self :: Node) -> dot :: Id < '_ > { dot :: Id :: new (format ! ("bb_{}" , n . index ())) . unwrap () } fn node_label (& self , block : & Self :: Node) -> dot :: LabelText < '_ > { let analysis = & mut * * self . analysis . borrow_mut () ; let diffs = StateDiffCollector :: run (self . body , * block , analysis , self . results , self . style) ; let mut fmt = BlockFormatter { cursor : ResultsCursor :: new_borrowing (self . body , analysis , self . results) , style : self . style , bg : Background :: Light , } ; let label = fmt . write_node_label (* block , diffs) . unwrap () ; dot :: LabelText :: html (String :: from_utf8 (label) . unwrap ()) } fn node_shape (& self , _n : & Self :: Node) -> Option < dot :: LabelText < '_ > > { Some (dot :: LabelText :: label ("none")) } fn edge_label (& self , e : & Self :: Edge) -> dot :: LabelText < '_ > { let label = & self . body [e . source] . terminator () . kind . fmt_successor_labels () [e . index] ; dot :: LabelText :: label (label . clone ()) } }
};
}
