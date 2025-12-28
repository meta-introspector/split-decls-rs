macro_rules! field_i {
    () => {
        fn field_i (i : usize) -> Ident { Ident :: new (& format ! ("__field{}" , i) , Span :: call_site ()) }
    };
}

field_i!();