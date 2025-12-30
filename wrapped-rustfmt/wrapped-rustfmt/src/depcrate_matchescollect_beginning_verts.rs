// Generated macro for collect_beginning_verts (function)
macro_rules! Depcrate_matchescollect_beginning_verts {
() => {
// Module: crate::matches
// Provides: {"collect_beginning_verts"}
// Dependencies: {}
# [doc = " Collect a byte position of the beginning `|` for each arm, if available."] fn collect_beginning_verts (context : & RewriteContext < '_ > , arms : & [ast :: Arm] ,) -> Vec < Option < BytePos > > { arms . iter () . map (| a | { context . snippet (a . pat . span) . starts_with ('|') . then (| | a . pat . span () . lo ()) }) . collect () }
};
}
