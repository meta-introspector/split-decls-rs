macro_rules! MapValueSerializer {
    () => {
        struct MapValueSerializer < 'd > { is_none : & 'd mut bool , }
    };
}

MapValueSerializer!()