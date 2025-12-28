macro_rules! allow_deprecated {
    () => {
        pub fn allow_deprecated (input : & syn :: DeriveInput) -> Option < TokenStream > { if should_allow_deprecated (input) { Some (quote ! { # [allow (deprecated)] }) } else { None } }
    };
}

allow_deprecated!();