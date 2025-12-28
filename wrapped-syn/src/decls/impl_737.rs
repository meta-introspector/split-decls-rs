macro_rules! deps {
    () => {
        TokenTreeHelper!();
        TokenStreamHelper!();
        Group!();
    };
}

macro_rules! impl_737 {
    () => {
        deps!();
        impl < 'a > PartialEq for TokenTreeHelper < 'a > { fn eq (& self , other : & Self) -> bool { match (self . 0 , other . 0) { (TokenTree :: Group (g1) , TokenTree :: Group (g2)) => { match (g1 . delimiter () , g2 . delimiter ()) { (Delimiter :: Parenthesis , Delimiter :: Parenthesis) | (Delimiter :: Brace , Delimiter :: Brace) | (Delimiter :: Bracket , Delimiter :: Bracket) | (Delimiter :: None , Delimiter :: None) => { } _ => return false , } TokenStreamHelper (& g1 . stream ()) == TokenStreamHelper (& g2 . stream ()) } (TokenTree :: Punct (o1) , TokenTree :: Punct (o2)) => { o1 . as_char () == o2 . as_char () && match (o1 . spacing () , o2 . spacing ()) { (Spacing :: Alone , Spacing :: Alone) | (Spacing :: Joint , Spacing :: Joint) => true , _ => false , } } (TokenTree :: Literal (l1) , TokenTree :: Literal (l2)) => l1 . to_string () == l2 . to_string () , (TokenTree :: Ident (s1) , TokenTree :: Ident (s2)) => s1 == s2 , _ => false , } } }
    };
}

impl_737!();