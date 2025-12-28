macro_rules! deps {
    () => {
        Const!();
    };
}

macro_rules! AdtDestructorKind {
    () => {
        deps!();
        # [doc = " Indicates that a `impl Drop for Adt` is `const` or not."] # [derive (Debug)] pub enum AdtDestructorKind { NotConst , Const , }
    };
}

AdtDestructorKind!();