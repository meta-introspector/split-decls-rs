macro_rules! deps {
    () => {
        Predefined!();
        Entries!();
        Symbol!();
        Errors!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl Entries { fn with_capacity (capacity : usize) -> Self { Entries { map : HashMap :: with_capacity (capacity) } } fn insert (& mut self , span : Span , s : & str , errors : & mut Errors) -> u32 { if let Some (prev) = self . map . get (s) { errors . error (span , format ! ("Symbol `{s}` is duplicated")) ; errors . error (prev . span_of_name , "location of previous definition" . to_string ()) ; prev . idx } else { let idx = self . len () ; self . map . insert (s . to_string () , Predefined { idx , span_of_name : span }) ; idx } } fn len (& self) -> u32 { u32 :: try_from (self . map . len ()) . expect ("way too many symbols") } }
    };
}

impl_137!()