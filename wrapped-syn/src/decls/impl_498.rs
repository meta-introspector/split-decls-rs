macro_rules! deps {
    () => {
        ParseBuffer!();
        Unexpected!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        impl < 'a > Drop for ParseBuffer < 'a > { fn drop (& mut self) { if let Some ((unexpected_span , delimiter)) = span_of_unexpected_ignoring_nones (self . cursor ()) { let (inner , old_span) = inner_unexpected (self) ; if old_span . is_none () { inner . set (Unexpected :: Some (unexpected_span , delimiter)) ; } } } }
    };
}

impl_498!();