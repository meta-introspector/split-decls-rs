macro_rules! deps {
    () => {
        Fragment!();
    };
}

macro_rules! Match {
    () => {
        deps!();
        # [doc = " Interpolate a fragment as the value part of a `match` expression. This"] # [doc = " involves putting a comma after expressions and curly braces around blocks."] pub struct Match (pub Fragment) ;
    };
}

Match!()