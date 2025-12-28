macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! MapValueSerializer {
    () => {
        deps!();
        pub (crate) struct MapValueSerializer < 'd > { dst : & 'd mut String , is_none : & 'd mut bool , style : Style , }
    };
}

MapValueSerializer!()