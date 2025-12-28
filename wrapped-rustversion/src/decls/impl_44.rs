macro_rules! deps {
    () => {
        IterImpl!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl Iterator for IterImpl { type Item = TokenTree ; fn next (& mut self) -> Option < Self :: Item > { if let Some (tt) = self . peeked . take () { return Some (tt) ; } loop { let top = self . stack . last_mut () ? ; match top . next () { None => drop (self . stack . pop ()) , Some (TokenTree :: Group (ref group)) if group . delimiter () == Delimiter :: None => { self . stack . push (group . stream () . into_iter ()) ; } Some (tt) => return Some (tt) , } } } }
    };
}

impl_44!();