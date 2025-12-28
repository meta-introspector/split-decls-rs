macro_rules! Tool {
    () => {
        # [derive (Copy , Clone)] pub enum Tool { Cargo , Rustc , Rustup , Rustfmt , }
    };
}

Tool!();