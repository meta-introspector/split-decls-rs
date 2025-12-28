macro_rules! deps {
    () => {
        Formatter!();
        DebugWithContext!();
    };
}

macro_rules! fmt_diff {
    () => {
        deps!();
        fn fmt_diff < T , C > (inserted : & MixedBitSet < T > , removed : & MixedBitSet < T > , ctxt : & C , f : & mut fmt :: Formatter < '_ > ,) -> fmt :: Result where T : Idx + DebugWithContext < C > , { let mut first = true ; for idx in inserted . iter () { let delim = if first { "\u{001f}+" } else if f . alternate () { "\n\u{001f}+" } else { ", " } ; write ! (f , "{delim}") ? ; idx . fmt_with (ctxt , f) ? ; first = false ; } if ! f . alternate () { first = true ; if ! inserted . is_empty () && ! removed . is_empty () { write ! (f , "\t") ? ; } } for idx in removed . iter () { let delim = if first { "\u{001f}-" } else if f . alternate () { "\n\u{001f}-" } else { ", " } ; write ! (f , "{delim}") ? ; idx . fmt_with (ctxt , f) ? ; first = false ; } Ok (()) }
    };
}

fmt_diff!();