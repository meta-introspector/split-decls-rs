macro_rules! deps {
    () => {
        IterMutImpl!();
    };
}

macro_rules! IterMut {
    () => {
        deps!();
        # [doc = " A mutable iterator over a `toml::Map`'s entries."] pub struct IterMut < 'a , K , V > { iter : IterMutImpl < 'a , K , V > , }
    };
}

IterMut!()