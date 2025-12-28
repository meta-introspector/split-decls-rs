macro_rules! deps {
    () => {
        Spacing!();
        Leaf!();
        Subtree!();
        DelimiterKind!();
        TokenTreesView!();
        Punct!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < S : Copy > fmt :: Display for TokenTreesView < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { return token_trees_display (f , self . iter ()) ; fn subtree_display < S > (subtree : & Subtree < S > , f : & mut fmt :: Formatter < '_ > , iter : TtIter < '_ , S > ,) -> fmt :: Result { let (l , r) = match subtree . delimiter . kind { DelimiterKind :: Parenthesis => ("(" , ")") , DelimiterKind :: Brace => ("{" , "}") , DelimiterKind :: Bracket => ("[" , "]") , DelimiterKind :: Invisible => ("" , "") , } ; f . write_str (l) ? ; token_trees_display (f , iter) ? ; f . write_str (r) ? ; Ok (()) } fn token_trees_display < S > (f : & mut fmt :: Formatter < '_ > , iter : TtIter < '_ , S >) -> fmt :: Result { let mut needs_space = false ; for child in iter { if needs_space { f . write_str (" ") ? ; } needs_space = true ; match child { TtElement :: Leaf (Leaf :: Punct (p)) => { needs_space = p . spacing == Spacing :: Alone ; fmt :: Display :: fmt (p , f) ? ; } TtElement :: Leaf (leaf) => fmt :: Display :: fmt (leaf , f) ? , TtElement :: Subtree (subtree , subtree_iter) => { subtree_display (subtree , f , subtree_iter) ? } } } Ok (()) } } }
    };
}

impl_25!()