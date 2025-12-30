// Generated macro for block_prefix (function)
macro_rules! Depcrate_exprblock_prefix {
() => {
// Module: crate::expr
// Provides: {"block_prefix"}
// Dependencies: {}
fn block_prefix (context : & RewriteContext < '_ > , block : & ast :: Block , shape : Shape) -> RewriteResult { Ok (match block . rules { ast :: BlockCheckMode :: Unsafe (..) => { let snippet = context . snippet (block . span) ; let open_pos = snippet . find_uncommented ("{") . unknown_error () ? ; let trimmed = & snippet [6 .. open_pos] . trim () ; if ! trimmed . is_empty () { let budget = shape . width . checked_sub (9) . max_width_error (shape . width , block . span) ? ; format ! ("unsafe {} " , rewrite_comment (trimmed , true , Shape :: legacy (budget , shape . indent + 7) , context . config ,) ?) } else { "unsafe " . to_owned () } } ast :: BlockCheckMode :: Default => String :: new () , }) }
};
}
