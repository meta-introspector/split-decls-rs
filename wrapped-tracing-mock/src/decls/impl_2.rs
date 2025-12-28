macro_rules! deps {
    () => {
        ExpectedAncestry!();
        ActualAncestry!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl ExpectedAncestry { # [track_caller] pub (crate) fn check (& self , actual_ancestry : & ActualAncestry , ctx : impl std :: fmt :: Display , collector_name : & str ,) { match (self , actual_ancestry) { (Self :: IsExplicitRoot , ActualAncestry :: IsExplicitRoot) => { } (Self :: IsContextualRoot , ActualAncestry :: IsContextualRoot) => { } (Self :: HasExplicitParent (expected_parent) , ActualAncestry :: HasExplicitParent (actual_parent) ,) => { expected_parent . check (actual_parent , format_args ! ("{ctx} to have an explicit parent span") , collector_name ,) ; } (Self :: HasContextualParent (expected_parent) , ActualAncestry :: HasContextualParent (actual_parent) ,) => { println ! ("----> [{collector_name}] check {expected_parent:?} against actual parent with Id={id:?}" , id = actual_parent . id ()) ; expected_parent . check (actual_parent , format_args ! ("{ctx} to have a contextual parent span") , collector_name ,) ; } _ => { let expected_description = match self { Self :: IsExplicitRoot => "be an explicit root" , Self :: HasExplicitParent (_) => "have an explicit parent span" , Self :: IsContextualRoot => "be a contextual root" , Self :: HasContextualParent (_) => "have a contextual parent span" , } ; let actual_description = match actual_ancestry { ActualAncestry :: IsExplicitRoot => "is actually an explicit root" , ActualAncestry :: HasExplicitParent (_) => "actually has an explicit parent span" , ActualAncestry :: IsContextualRoot => "is actually a contextual root" , ActualAncestry :: HasContextualParent (_) => { "actually has a contextual parent span" } } ; panic ! ("{}" , format ! ("[{collector_name}] expected {ctx} to {expected_description}, \
                        but it {actual_description}")) ; } } } }
    };
}

impl_2!();