macro_rules! deps {
    () => {
        SyntaxElement!();
    };
}

macro_rules! remove_all_iter {
    () => {
        deps!();
        pub fn remove_all_iter (range : impl IntoIterator < Item = SyntaxElement >) { let mut it = range . into_iter () ; if let Some (mut first) = it . next () { match it . last () { Some (mut last) => { if first . index () > last . index () { mem :: swap (& mut first , & mut last) ; } remove_all (first ..= last) ; } None => remove (first) , } } }
    };
}

remove_all_iter!()