macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! macro_15 {
    () => {
        deps!();
        ops ! (impl Sub for TextRange by fn sub = -) ;
    };
}

macro_15!();