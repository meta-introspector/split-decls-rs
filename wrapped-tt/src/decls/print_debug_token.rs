macro_rules! deps {
    () => {
        Leaf!();
        Subtree!();
        Ident!();
        Literal!();
        TtElement!();
        Spacing!();
        Punct!();
    };
}

macro_rules! print_debug_token {
    () => {
        deps!();
        fn print_debug_token < S : fmt :: Debug > (f : & mut fmt :: Formatter < '_ > , level : usize , tt : TtElement < '_ , S > ,) -> fmt :: Result { let align = "  " . repeat (level) ; match tt { TtElement :: Leaf (leaf) => match leaf { Leaf :: Literal (lit) => { write ! (f , "{}LITERAL {:?} {}{} {:#?}" , align , lit . kind , lit . symbol , lit . suffix . as_ref () . map (| it | it . as_str ()) . unwrap_or ("") , lit . span) ? ; } Leaf :: Punct (punct) => { write ! (f , "{}PUNCH   {} [{}] {:#?}" , align , punct . char , if punct . spacing == Spacing :: Alone { "alone" } else { "joint" } , punct . span) ? ; } Leaf :: Ident (ident) => { write ! (f , "{}IDENT   {}{} {:#?}" , align , ident . is_raw . as_str () , ident . sym , ident . span) ? ; } } , TtElement :: Subtree (subtree , subtree_iter) => { print_debug_subtree (f , subtree , level , subtree_iter) ? ; } } Ok (()) }
    };
}

print_debug_token!();