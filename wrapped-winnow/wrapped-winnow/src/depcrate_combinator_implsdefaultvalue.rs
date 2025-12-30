// Generated macro for DefaultValue (struct)
macro_rules! Depcrate_combinator_implsDefaultValue {
() => {
// Module: crate::combinator::impls
// Provides: {"DefaultValue"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::default_value`]"] pub struct DefaultValue < F , I , O , O2 , E > where F : Parser < I , O , E > , O2 : core :: default :: Default , { pub (crate) parser : F , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
