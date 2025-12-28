macro_rules! RuleKind {
    () => {
        # [derive (Debug , Deserialize)] pub enum RuleKind { # [serde (rename = "add_derive")] AddDerive , # [serde (rename = "add_impl")] AddImpl , # [serde (rename = "add_impl_default")] AddImplDefault , # [serde (rename = "remove_impl")] RemoveImpl , }
    };
}

RuleKind!()