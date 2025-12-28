macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! macro_28 {
    () => {
        deps!();
        ops ! (impl Add for TextSize by fn add = +) ;
    };
}

macro_28!();