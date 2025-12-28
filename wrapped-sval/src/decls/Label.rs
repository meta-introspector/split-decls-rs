macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! Label {
    () => {
        deps!();
        # [doc = "\nA textual label for some value.\n"] pub struct Label < 'computed > { value_computed : * const str , backing_field_static : Option < & 'static str > , # [cfg (feature = "alloc")] backing_field_owned : Option < * mut str > , tag : Option < Tag > , _marker : PhantomData < & 'computed str > , }
    };
}

Label!();