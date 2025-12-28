macro_rules! deps {
    () => {
        NewSpan!();
        ActualAncestry!();
        ExpectedAncestry!();
        ExpectedFields!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl NewSpan { # [doc = " Configures this `NewSpan` to expect the specified [`ExpectedAncestry`]."] # [doc = " A span's ancestry indicates whether it has a parent or is a root span"] # [doc = " and whether the parent is explitly or contextually assigned."] # [doc = ""] # [doc = " For more information and examples, see the documentation on"] # [doc = " [`ExpectedSpan::with_ancestry`]."] pub fn with_ancestry (self , ancestry : ExpectedAncestry) -> NewSpan { NewSpan { ancestry : Some (ancestry) , .. self } } # [doc = " Adds fields to expect when matching a span."] # [doc = ""] # [doc = " For more information and examples, see the documentation on"] # [doc = " [`ExpectedSpan::with_fields`]."] # [doc = ""] # [doc = " [`ExpectedSpan::with_fields`]: fn@crate::span::ExpectedSpan::with_fields"] pub fn with_fields < I > (self , fields : I) -> NewSpan where I : Into < ExpectedFields > , { NewSpan { fields : fields . into () , .. self } } pub (crate) fn check (& mut self , span : & tracing_core :: span :: Attributes < '_ > , get_ancestry : impl FnOnce () -> ActualAncestry , subscriber_name : & str ,) { let meta = span . metadata () ; let name = meta . name () ; self . span . metadata . check (meta , "a new span" , subscriber_name) ; let mut checker = self . fields . checker (name , subscriber_name) ; span . record (& mut checker) ; checker . finish () ; if let Some (ref expected_ancestry) = self . ancestry { let actual_ancestry = get_ancestry () ; expected_ancestry . check (& actual_ancestry , format_args ! ("span `{}`" , name) , subscriber_name ,) ; } } }
    };
}

impl_57!()