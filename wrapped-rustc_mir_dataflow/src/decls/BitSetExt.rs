macro_rules! deps {
    () => {
        Analysis!();
    };
}

macro_rules! BitSetExt {
    () => {
        deps!();
        # [doc = " Analysis domains are all bitsets of various kinds. This trait holds"] # [doc = " operations needed by all of them."] pub trait BitSetExt < T > { fn contains (& self , elem : T) -> bool ; }
    };
}

BitSetExt!();