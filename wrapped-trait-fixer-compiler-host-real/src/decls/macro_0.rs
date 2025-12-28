macro_rules! macro_0 {
    () => {
        include ! (concat ! (env ! ("OUT_DIR") , "/generated_lib.rs")) ;
    };
}

macro_0!();