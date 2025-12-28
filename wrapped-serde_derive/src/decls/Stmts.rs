macro_rules! deps {
    () => {
        Fragment!();
    };
}

macro_rules! Stmts {
    () => {
        deps!();
        # [doc = " Interpolate a fragment as the statements of a block."] pub struct Stmts (pub Fragment) ;
    };
}

Stmts!();