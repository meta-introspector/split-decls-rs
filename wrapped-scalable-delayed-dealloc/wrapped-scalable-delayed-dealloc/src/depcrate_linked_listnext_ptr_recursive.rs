// Generated macro for next_ptr_recursive (function)
macro_rules! Depcrate_linked_listnext_ptr_recursive {
() => {
// Module: crate::linked_list
// Provides: {"next_ptr_recursive"}
// Dependencies: {}
# [doc = " Recursively cleans up the linked list starting from the supplied head."] fn next_ptr_recursive < 'g , T : LinkedList > (head : & T , order : Ordering , mut depth : usize , guard : & 'g Guard ,) -> Ptr < 'g , T > { let mut head_next_ptr = head . link_ref () . load (order , guard) ; let mut head_tag = head_next_ptr . tag () ; loop { let mut next_ptr = head_next_ptr ; let next_valid_ptr = loop { if let Some (next_ref) = next_ptr . as_ref () { let next_next_ptr = next_ref . link_ref () . load (order , guard) ; if next_next_ptr . tag () != Tag :: Second { break next_ptr ; } if depth != 0 { break next_ptr_recursive (next_ref , order , depth - 1 , guard) ; } next_ptr = next_next_ptr ; } else { break Ptr :: null () ; } } ; depth = 0 ; if next_valid_ptr . with_tag (head_tag) != head_next_ptr { let next_valid_entry = next_valid_ptr . get_shared () ; if ! next_valid_ptr . is_null () && next_valid_entry . is_none () { head_next_ptr = head . link_ref () . load (order , guard) ; head_tag = head_next_ptr . tag () ; continue ; } match head . link_ref () . compare_exchange (head_next_ptr , (next_valid_entry , head_tag) , AcqRel , Acquire , guard ,) { Ok ((prev , _)) => { if let Some (removed) = prev { let _ : bool = removed . release () ; } } Err ((_ , actual)) => { head_next_ptr = actual ; head_tag = actual . tag () ; continue ; } } } return next_valid_ptr ; } }
};
}
