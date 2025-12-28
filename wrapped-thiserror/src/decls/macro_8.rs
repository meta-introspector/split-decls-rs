macro_rules! macro_8 {
    () => {
        include ! (concat ! (env ! ("OUT_DIR") , "/private.rs")) ;
    };
}

macro_8!()