// Generated macro for OutputInto (struct)
macro_rules! Depcrate_combinator_implsOutputInto {
() => {
// Module: crate::combinator::impls
// Provides: {"OutputInto"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::output_into`]"] pub struct OutputInto < F , I , O , O2 , E > where F : Parser < I , O , E > , O : Into < O2 > , { pub (crate) parser : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
