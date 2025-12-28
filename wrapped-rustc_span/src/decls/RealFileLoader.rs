macro_rules! deps {
    () => {
        FileLoader!();
    };
}

macro_rules! RealFileLoader {
    () => {
        deps!();
        # [doc = " A FileLoader that uses std::fs to load real files."] pub struct RealFileLoader ;
    };
}

RealFileLoader!();