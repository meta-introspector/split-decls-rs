macro_rules! deps {
    () => {
        Scope!();
        LookupSpan!();
        SpanRef!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'a , R > Iterator for Scope < 'a , R > where R : LookupSpan < 'a > , { type Item = SpanRef < 'a , R > ; fn next (& mut self) -> Option < Self :: Item > { loop { let curr = self . registry . span (self . next . as_ref () ?) ? ; # [cfg (all (feature = "registry" , feature = "std"))] let curr = curr . with_filter (self . filter) ; self . next = curr . data . parent () . cloned () ; # [cfg (all (feature = "registry" , feature = "std"))] { if ! curr . is_enabled_for (self . filter) { continue ; } } return Some (curr) ; } } }
    };
}

impl_89!();