macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! macro_14 {
    () => {
        deps!();
        ops ! (impl Add for TextRange by fn add = +) ;
    };
}

macro_14!();