macro_rules! deps {
    () => {
        Leaf!();
        Spacing!();
        Punct!();
        TopSubtree!();
        DelimiterKind!();
        TokenTree!();
        Ident!();
        Subtree!();
        Literal!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < S > TopSubtree < S > { # [doc = " A simple line string used for debugging"] pub fn subtree_as_debug_string (& self , subtree_idx : usize) -> String { fn debug_subtree < S > (output : & mut String , subtree : & Subtree < S > , iter : & mut std :: slice :: Iter < '_ , TokenTree < S > > ,) { let delim = match subtree . delimiter . kind { DelimiterKind :: Brace => ("{" , "}") , DelimiterKind :: Bracket => ("[" , "]") , DelimiterKind :: Parenthesis => ("(" , ")") , DelimiterKind :: Invisible => ("$" , "$") , } ; output . push_str (delim . 0) ; let mut last = None ; let mut idx = 0 ; while idx < subtree . len { let child = iter . next () . unwrap () ; debug_token_tree (output , child , last , iter) ; last = Some (child) ; idx += 1 ; } output . push_str (delim . 1) ; } fn debug_token_tree < S > (output : & mut String , tt : & TokenTree < S > , last : Option < & TokenTree < S > > , iter : & mut std :: slice :: Iter < '_ , TokenTree < S > > ,) { match tt { TokenTree :: Leaf (it) => { let s = match it { Leaf :: Literal (it) => it . symbol . to_string () , Leaf :: Punct (it) => it . char . to_string () , Leaf :: Ident (it) => format ! ("{}{}" , it . is_raw . as_str () , it . sym) , } ; match (it , last) { (Leaf :: Ident (_) , Some (& TokenTree :: Leaf (Leaf :: Ident (_)))) => { output . push (' ') ; output . push_str (& s) ; } (Leaf :: Punct (_) , Some (TokenTree :: Leaf (Leaf :: Punct (punct)))) => { if punct . spacing == Spacing :: Alone { output . push (' ') ; output . push_str (& s) ; } else { output . push_str (& s) ; } } _ => output . push_str (& s) , } } TokenTree :: Subtree (it) => debug_subtree (output , it , iter) , } } let mut res = String :: new () ; debug_token_tree (& mut res , & self . 0 [subtree_idx] , None , & mut self . 0 [subtree_idx + 1 ..] . iter () ,) ; res } }
    };
}

impl_62!()