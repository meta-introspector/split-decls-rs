macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! t {
    () => {
        deps!();
        # [track_caller] fn t (input : & str , expected : impl IntoData) { let source = crate :: Source :: new (input) ; let actual = source . lex () . into_vec () ; assert_data_eq ! (actual . to_debug () , expected) ; if ! actual . is_empty () { let spans = actual . iter () . map (| t | t . span ()) . collect :: < Vec < _ > > () ; assert_eq ! (spans . first () . unwrap () . start () , 0) ; assert_eq ! (spans . last () . unwrap () . end () , input . len ()) ; for i in 0 .. (spans . len () - 1) { let current = & spans [i] ; let next = & spans [i + 1] ; assert_eq ! (current . end () , next . start ()) ; } } }
    };
}

t!()