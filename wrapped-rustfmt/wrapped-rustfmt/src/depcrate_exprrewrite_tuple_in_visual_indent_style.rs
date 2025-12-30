// Generated macro for rewrite_tuple_in_visual_indent_style (function)
macro_rules! Depcrate_exprrewrite_tuple_in_visual_indent_style {
() => {
// Module: crate::expr
// Provides: {"rewrite_tuple_in_visual_indent_style"}
// Dependencies: {}
fn rewrite_tuple_in_visual_indent_style < 'a , T : 'a + IntoOverflowableItem < 'a > > (context : & RewriteContext < '_ > , mut items : impl Iterator < Item = & 'a T > , span : Span , shape : Shape , is_singleton_tuple : bool ,) -> RewriteResult { debug ! ("rewrite_tuple_in_visual_indent_style {:?}" , shape) ; if is_singleton_tuple { let nested_shape = shape . sub_width (3 , span) ? . visual_indent (1) ; return items . next () . unwrap () . rewrite_result (context , nested_shape) . map (| s | format ! ("({},)" , s)) ; } let list_lo = context . snippet_provider . span_after (span , "(") ; let nested_shape = shape . sub_width (2 , span) ? . visual_indent (1) ; let items = itemize_list (context . snippet_provider , items , ")" , "," , | item | item . span () . lo () , | item | item . span () . hi () , | item | item . rewrite_result (context , nested_shape) , list_lo , span . hi () - BytePos (1) , false ,) ; let item_vec : Vec < _ > = items . collect () ; let tactic = definitive_tactic (& item_vec , ListTactic :: HorizontalVertical , Separator :: Comma , nested_shape . width ,) ; let fmt = ListFormatting :: new (nested_shape , context . config) . tactic (tactic) . ends_with_newline (false) ; let list_str = write_list (& item_vec , & fmt) ? ; Ok (format ! ("({list_str})")) }
};
}
