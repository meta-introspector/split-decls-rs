macro_rules! macro_35 {
    () => {
        include ! (concat ! (env ! ("OUT_DIR") , "/private.rs")) ;
    };
}

macro_35!()