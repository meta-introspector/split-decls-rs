macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! assert_lex_rule {
    () => {
        deps!();
        macro_rules ! assert_lex_rule { ($ rule : expr , $ input : expr) => { let res = TeraParser :: parse ($ rule , $ input) ; println ! ("{:?}" , $ input) ; println ! ("{:#?}" , res) ; if res . is_err () { println ! ("{}" , res . unwrap_err ()) ; panic ! () ; } assert ! (res . is_ok ()) ; assert_eq ! (res . unwrap () . last () . unwrap () . as_span () . end () , $ input . len ()) ; } ; }
    };
}

assert_lex_rule!()