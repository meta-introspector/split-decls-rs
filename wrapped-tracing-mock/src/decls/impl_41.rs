macro_rules! deps {
    () => {
        ExpectedMetadata!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl ExpectedMetadata { # [doc = " Checks the given metadata against this expected metadata and panics if"] # [doc = " there is a mismatch."] # [doc = ""] # [doc = " The context `ctx` should fit into the followint sentence:"] # [doc = ""] # [doc = " > expected {ctx} named `expected_name`, but got one named `actual_name`"] # [doc = ""] # [doc = " Examples could be:"] # [doc = " * a new span"] # [doc = " * to enter a span"] # [doc = " * an event"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This method will panic if any of the expectations that have been"] # [doc = " specified are noto met."] # [doc = ""] pub (crate) fn check (& self , actual : & Metadata < '_ > , ctx : impl fmt :: Display , subscriber_name : & str ,) { if let Some (ref expected_name) = self . name { let actual_name = actual . name () ; assert ! (expected_name == actual_name , "{}" , format_args ! ("\n[{subscriber_name}] expected {ctx} named `{expected_name}`,\n\
                    [{subscriber_name}] but got one named `{actual_name}` instead.") ,) } if let Some (ref expected_level) = self . level { let actual_level = actual . level () ; assert ! (expected_level == actual_level , "{}" , format_args ! ("\n[{subscriber_name}] expected {ctx} at level `{expected_level:?}`,\n\
                    [{subscriber_name}] but got one at level `{actual_level:?}` instead.") ,) } if let Some (ref expected_target) = self . target { let actual_target = actual . target () ; assert ! (expected_target == actual_target , "{}" , format_args ! ("\n[{subscriber_name}] expected {ctx} with target `{expected_target}`,\n\
                    [{subscriber_name}] but got one with target `{actual_target}` instead.") ,) } } pub (crate) fn has_expectations (& self) -> bool { self . name . is_some () || self . level . is_some () || self . target . is_some () } }
    };
}

impl_41!()