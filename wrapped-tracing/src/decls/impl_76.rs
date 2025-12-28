macro_rules! deps {
    () => {
        EnteredSpan!();
        Span!();
        Inner!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl EnteredSpan { # [doc = " Returns this span's `Id`, if it is enabled."] pub fn id (& self) -> Option < Id > { self . inner . as_ref () . map (Inner :: id) } # [doc = " Exits this span, returning the underlying [`Span`]."] # [inline] pub fn exit (mut self) -> Span { let span = mem :: replace (& mut self . span , Span :: none ()) ; span . do_exit () ; span } }
    };
}

impl_76!()