macro_rules! deps {
    () => {
        EffectIndex!();
    };
}

macro_rules! CursorPosition {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] struct CursorPosition { block : BasicBlock , curr_effect_index : Option < EffectIndex > , }
    };
}

CursorPosition!()