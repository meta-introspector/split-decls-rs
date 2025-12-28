macro_rules! hash_stable_body {
    () => {
        fn hash_stable_body (s : & mut synstructure :: Structure < '_ >) -> proc_macro2 :: TokenStream { s . each (| bi | { let attrs = parse_attributes (bi . ast ()) ; if attrs . ignore { quote ! { } } else if let Some (project) = attrs . project { quote ! { (&# bi .# project) . hash_stable (__hcx , __hasher) ; } } else { quote ! { # bi . hash_stable (__hcx , __hasher) ; } } }) }
    };
}

hash_stable_body!()