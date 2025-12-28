macro_rules! deps {
    () => {
        MockConfig!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl ConfigTrait for MockConfig { fn load () -> Self { MockConfig { rule : vec ! [Rule { kind : RuleKind :: AddDerive , trait_name : vec ! ["Debug" . to_string () , "PartialEq" . to_string ()] , apply_to : vec ! [] , condition : "true" . to_string () , }] , } } fn get_rules (& self) -> & Vec < Rule > { & self . rule } }
    };
}

impl_1!();