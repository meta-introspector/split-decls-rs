macro_rules! deps {
    () => {
        MapImpl!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        # [doc = " Represents a TOML key/value type."] pub struct Map < K , V > { map : MapImpl < K , V > , dotted : bool , implicit : bool , inline : bool , }
    };
}

Map!();