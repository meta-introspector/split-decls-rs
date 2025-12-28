macro_rules! deps {
    () => {
        KeysImpl!();
    };
}

macro_rules! Keys {
    () => {
        deps!();
        # [doc = " An iterator over a `toml::Map`'s keys."] pub struct Keys < 'a , K , V > { iter : KeysImpl < 'a , K , V > , }
    };
}

Keys!();