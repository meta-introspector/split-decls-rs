macro_rules! to_raw {
    () => {
        fn to_raw < T > (data : Option < Box < T > >) -> * mut T { data . map_or (ptr :: null_mut () , Box :: into_raw) }
    };
}

to_raw!();