macro_rules! deps {
    () => {
        StreamStatus!();
        Stream!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Stream { fn make_text (mut self) -> Self { let content = self . content . coerce_to (DataFormat :: Text) ; if content . format () != DataFormat :: Text { self . status = StreamStatus :: Failure ("Unable to convert underlying Data to Text" . into ()) ; } self . content = FilterNewlines . filter (FilterPaths . filter (content)) ; self } fn is_ok (& self) -> bool { self . status . is_ok () } }
    };
}

impl_64!()