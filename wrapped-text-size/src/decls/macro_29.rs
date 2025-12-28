macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! macro_29 {
    () => {
        deps!();
        ops ! (impl Sub for TextSize by fn sub = -) ;
    };
}

macro_29!();