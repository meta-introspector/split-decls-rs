macro_rules! deps {
    () => {
        KeyMetrics!();
    };
}

macro_rules! TomlKeyBuilder {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub struct TomlKeyBuilder < 's > { decoded : & 's str , metrics : KeyMetrics , }
    };
}

TomlKeyBuilder!();