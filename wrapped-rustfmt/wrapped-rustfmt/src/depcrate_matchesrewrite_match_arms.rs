// Generated macro for rewrite_match_arms (function)
macro_rules! Depcrate_matchesrewrite_match_arms {
() => {
// Module: crate::matches
// Provides: {"rewrite_match_arms"}
// Dependencies: {}
fn rewrite_match_arms (context : & RewriteContext < '_ > , arms : & [ast :: Arm] , shape : Shape , span : Span , open_brace_pos : BytePos ,) -> RewriteResult { let arm_shape = if context . config . match_arm_indent () { shape . block_indent (context . config . tab_spaces ()) } else { shape } . with_max_width (context . config) ; let arm_len = arms . len () ; let is_last_iter = repeat (false) . take (arm_len . saturating_sub (1)) . chain (repeat (true)) ; let beginning_verts = collect_beginning_verts (context , arms) ; let items = itemize_list (context . snippet_provider , arms . iter () . zip (is_last_iter) . zip (beginning_verts . into_iter ()) . map (| ((arm , is_last) , beginning_vert) | ArmWrapper :: new (arm , is_last , beginning_vert)) , "}" , "|" , | arm | arm . span () . lo () , | arm | arm . span () . hi () , | arm | arm . rewrite_result (context , arm_shape) , open_brace_pos , span . hi () , false ,) ; let arms_vec : Vec < _ > = items . collect () ; let fmt = ListFormatting :: new (arm_shape , context . config) . separator ("") . preserve_newline (true) ; write_list (& arms_vec , & fmt) }
};
}
