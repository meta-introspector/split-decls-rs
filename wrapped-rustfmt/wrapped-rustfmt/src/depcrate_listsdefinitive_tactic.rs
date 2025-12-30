// Generated macro for definitive_tactic (function)
macro_rules! Depcrate_listsdefinitive_tactic {
() => {
// Module: crate::lists
// Provides: {"definitive_tactic"}
// Dependencies: {}
pub (crate) fn definitive_tactic < I , T > (items : I , tactic : ListTactic , sep : Separator , width : usize ,) -> DefinitiveListTactic where I : IntoIterator < Item = T > + Clone , T : AsRef < ListItem > , { let pre_line_comments = items . clone () . into_iter () . any (| item | item . as_ref () . has_single_line_comment ()) ; let limit = match tactic { _ if pre_line_comments => return DefinitiveListTactic :: Vertical , ListTactic :: Horizontal => return DefinitiveListTactic :: Horizontal , ListTactic :: Vertical => return DefinitiveListTactic :: Vertical , ListTactic :: LimitedHorizontalVertical (limit) => :: std :: cmp :: min (width , limit) , ListTactic :: Mixed | ListTactic :: HorizontalVertical => width , } ; let (sep_count , total_width) = calculate_width (items . clone ()) ; let total_sep_len = sep . len () * sep_count . saturating_sub (1) ; let real_total = total_width + total_sep_len ; if real_total <= limit && ! items . into_iter () . any (| item | item . as_ref () . is_multiline ()) { DefinitiveListTactic :: Horizontal } else { match tactic { ListTactic :: Mixed => DefinitiveListTactic :: Mixed , _ => DefinitiveListTactic :: Vertical , } } }
};
}
