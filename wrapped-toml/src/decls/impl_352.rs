macro_rules! deps {
    () => {
        MapValueSerializer!();
        Style!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl < 'd > MapValueSerializer < 'd > { pub (crate) fn new (dst : & 'd mut String , is_none : & 'd mut bool , style : Style) -> Self { Self { dst , is_none , style , } } }
    };
}

impl_352!()