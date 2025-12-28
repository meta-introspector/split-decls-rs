macro_rules! deps {
    () => {
        Lint!();
    };
}

macro_rules! LintId {
    () => {
        deps!();
        # [doc = " Identifies a lint known to the compiler."] # [derive (Clone , Copy , Debug)] pub struct LintId { pub lint : & 'static Lint , }
    };
}

LintId!()