macro_rules! deps {
    () => {
        Palette!();
        Action!();
        Redactions!();
    };
}

macro_rules! Assert {
    () => {
        deps!();
        # [doc = " Snapshot assertion against a file's contents"] # [doc = ""] # [doc = " Useful for one-off assertions with the snapshot stored in a file"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use snapbox::Assert;"] # [doc = " # use snapbox::file;"] # [doc = " let actual = \"something\";"] # [doc = " Assert::new().eq(actual, file![\"output.txt\"]);"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Assert { pub (crate) action : Action , action_var : Option < String > , normalize_paths : bool , substitutions : crate :: Redactions , pub (crate) palette : crate :: report :: Palette , }
    };
}

Assert!();