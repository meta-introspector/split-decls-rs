macro_rules! deps {
    () => {
        SliceKind!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl SliceKind { pub fn arity (self) -> usize { match self { FixedLen (length) => length , VarLen (prefix , suffix) => prefix + suffix , } } # [doc = " Whether this pattern includes patterns of length `other_len`."] fn covers_length (self , other_len : usize) -> bool { match self { FixedLen (len) => len == other_len , VarLen (prefix , suffix) => prefix + suffix <= other_len , } } }
    };
}

impl_12!();