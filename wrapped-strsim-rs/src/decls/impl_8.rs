macro_rules! deps {
    () => {
        StringWrapper!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , 'b > IntoIterator for & 'a StringWrapper < 'b > { type Item = char ; type IntoIter = Chars < 'b > ; fn into_iter (self) -> Self :: IntoIter { self . 0 . chars () } }
    };
}

impl_8!()