macro_rules! DocCommentDesugarMode {
    () => {
        # [doc = " Doc comment desugaring differs between mbe and proc-macros."] # [derive (Copy , Clone , PartialEq , Eq)] pub enum DocCommentDesugarMode { # [doc = " Desugars doc comments as quoted raw strings"] Mbe , # [doc = " Desugars doc comments as quoted strings"] ProcMacro , }
    };
}

DocCommentDesugarMode!()