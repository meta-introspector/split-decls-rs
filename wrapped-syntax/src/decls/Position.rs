macro_rules! deps {
    () => {
        PositionRepr!();
    };
}

macro_rules! Position {
    () => {
        deps!();
        # [derive (Debug)] pub struct Position { repr : PositionRepr , }
    };
}

Position!()