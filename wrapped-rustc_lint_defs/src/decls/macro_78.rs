macro_rules! macro_78 {
    () => {
        declare_lint ! { # [doc = " The `large_assignments` lint detects when objects of large"] # [doc = " types are being moved around."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,ignore (can crash on some platforms)"] # [doc = " let x = [0; 50000];"] # [doc = " let y = x;"] # [doc = " ```"] # [doc = ""] # [doc = " produces:"] # [doc = ""] # [doc = " ```text"] # [doc = " warning: moving a large value"] # [doc = "   --> $DIR/move-large.rs:1:3"] # [doc = "   let y = x;"] # [doc = "           - Copied large value here"] # [doc = " ```"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " When using a large type in a plain assignment or in a function"] # [doc = " argument, idiomatic code can be inefficient."] # [doc = " Ideally appropriate optimizations would resolve this, but such"] # [doc = " optimizations are only done in a best-effort manner."] # [doc = " This lint will trigger on all sites of large moves and thus allow the"] # [doc = " user to resolve them in code."] pub LARGE_ASSIGNMENTS , Warn , "detects large moves or copies" , }
    };
}

macro_78!()