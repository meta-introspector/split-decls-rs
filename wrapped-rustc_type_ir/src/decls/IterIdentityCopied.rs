macro_rules! IterIdentityCopied {
    () => {
        pub struct IterIdentityCopied < Iter : IntoIterator > { it : Iter :: IntoIter , }
    };
}

IterIdentityCopied!();