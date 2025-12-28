macro_rules! deps {
    () => {
        Adler32!();
    };
}

macro_rules! Adler32Hash {
    () => {
        deps!();
        # [doc = " A Adler-32 hash-able type."] pub trait Adler32Hash { # [doc = " Feeds this value into `Adler32`."] fn hash (& self) -> u32 ; }
    };
}

Adler32Hash!()