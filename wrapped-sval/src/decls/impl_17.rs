macro_rules! deps {
    () => {
        Label!();
        Tag!();
        Error!();
        Result!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'computed > Label < 'computed > { # [doc = "\n    Create a new label from a static string value.\n\n    For labels that can't satisfy the `'static` lifetime, use [`Label::new_computed`].\n    For labels that need owned values, use [`Label::new_owned`].\n    "] # [inline (always)] pub const fn new (label : & 'static str) -> Self { Label { value_computed : label as * const str , backing_field_static : Some (label) , # [cfg (feature = "alloc")] backing_field_owned : None , tag : None , _marker : PhantomData , } } # [doc = "\n    Create a new label from a string value borrowed for the `'computed` lifetime.\n    "] # [inline (always)] pub const fn new_computed (label : & 'computed str) -> Self { Label { value_computed : label as * const str , backing_field_static : None , # [cfg (feature = "alloc")] backing_field_owned : None , tag : None , _marker : PhantomData , } } # [doc = "\n    Get the value of the label as a string.\n    "] # [inline (always)] pub const fn as_str (& self) -> & str { unsafe { & * self . value_computed } } # [doc = "\n    Try get the value of the label as a static string.\n\n    For labels that were created over computed data this method will return `None`.\n    "] # [inline (always)] pub const fn as_static_str (& self) -> Option < & 'static str > { self . backing_field_static } # [doc = "\n    Associate a tag as a hint with this label.\n\n    Tags don't contribute to equality or ordering of labels but streams may\n    use the them when interpreting the label value.\n     "] # [inline (always)] pub const fn with_tag (mut self , tag : & Tag) -> Self { self . tag = Some (tag . cloned ()) ; self } # [doc = "\n    Get the tag hint associated with the label, if present.\n\n    Streams may use the tag when interpreting the label value.\n     "] # [inline (always)] pub const fn tag (& self) -> Option < & Tag > { self . tag . as_ref () } # [doc = "\n    Try create an owned label from this one.\n\n    This method will always return `Ok` if the `alloc` feature is enabled.\n    If the `alloc` feature is not enabled then this method will only return `Ok`\n    if the underlying value is already `'static`.\n    "] # [inline (always)] pub fn try_to_owned (& self) -> Result < Label < 'static > > { # [cfg (feature = "alloc")] { Ok (self . to_owned ()) } # [cfg (not (feature = "alloc"))] { self . as_static_str () . map (Label :: new) . ok_or_else (crate :: Error :: new) } } }
    };
}

impl_17!();