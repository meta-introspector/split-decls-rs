// Generated macro for group_aligned_items (function)
macro_rules! Depcrate_verticalgroup_aligned_items {
() => {
// Module: crate::vertical
// Provides: {"group_aligned_items"}
// Dependencies: {}
# [doc = " Returns the index in `fields` up to which a field belongs to the current group."] # [doc = " The returned string is the group separator to use when rewriting the fields."] # [doc = " Groups are defined by blank lines."] fn group_aligned_items < T : AlignedItem > (context : & RewriteContext < '_ > , fields : & [T] ,) -> (& 'static str , usize) { let mut index = 0 ; for i in 0 .. fields . len () - 1 { if fields [i] . skip () { return ("" , index) ; } let span = mk_sp (fields [i] . get_span () . hi () , fields [i + 1] . get_span () . lo ()) ; let snippet = context . snippet (span) . lines () . skip (1) . collect :: < Vec < _ > > () . join ("\n") ; let has_blank_line = snippet . lines () . dropping_back (1) . any (| l | l . trim () . is_empty ()) ; if has_blank_line { return ("\n" , index) ; } index += 1 ; } ("" , index) }
};
}
