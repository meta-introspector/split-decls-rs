macro_rules! deps {
    () => {
        ValuesImpl!();
    };
}

macro_rules! Values {
    () => {
        deps!();
        # [doc = " An iterator over a `toml::Map`'s values."] pub struct Values < 'a , K , V > { iter : ValuesImpl < 'a , K , V > , }
    };
}

Values!();