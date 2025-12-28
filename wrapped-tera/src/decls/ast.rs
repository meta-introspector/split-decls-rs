macro_rules! deps {
    () => {
        Tera!();
    };
}

macro_rules! ast {
    () => {
        deps!();
        # [doc = " The AST of Tera"] pub mod ast ;
    };
}

ast!()