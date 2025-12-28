macro_rules! strips_trailing_slash {
    () => {
        # [test] fn strips_trailing_slash () { let path = std :: path :: Path :: new ("/foo/bar/") ; let rendered = path . display () . to_string () ; assert_eq ! (rendered . as_bytes () [rendered . len () - 1] , b'/') ; let stripped = strip_trailing_slash (path) ; let rendered = stripped . display () . to_string () ; assert_eq ! (rendered . as_bytes () [rendered . len () - 1] , b'r') ; }
    };
}

strips_trailing_slash!();