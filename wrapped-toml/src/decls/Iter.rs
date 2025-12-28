macro_rules! deps {
    () => {
        IterImpl!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over a `toml::Map`'s entries."] pub struct Iter < 'a , K , V > { iter : IterImpl < 'a , K , V > , }
    };
}

Iter!()