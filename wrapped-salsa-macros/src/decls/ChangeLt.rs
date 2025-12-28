macro_rules! ChangeLt {
    () => {
        pub (crate) struct ChangeLt < 'a > { from : Option < & 'a str > , to : String , }
    };
}

ChangeLt!()