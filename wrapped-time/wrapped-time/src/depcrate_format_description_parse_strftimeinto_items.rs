// Generated macro for into_items (function)
macro_rules! Depcrate_format_description_parse_strftimeinto_items {
() => {
// Module: crate::format_description::parse::strftime
// Provides: {"into_items"}
// Dependencies: {}
# [inline] fn into_items < 'iter , 'token : 'iter > (mut tokens : iter :: Peekable < impl Iterator < Item = Result < Token < 'token > , Error > > + 'iter > ,) -> impl Iterator < Item = Result < BorrowedFormatItem < 'token > , Error > > + 'iter { iter :: from_fn (move | | { let next = match tokens . next () ? { Ok (token) => token , Err (err) => return Some (Err (err)) , } ; Some (match next { Token :: Literal (spanned) => Ok (BorrowedFormatItem :: Literal (* spanned)) , Token :: Component { _percent , padding , component , } => parse_component (padding , component) , }) }) }
};
}
