macro_rules! RegPassKind {
    () => {
        # [derive (Copy , Clone)] enum RegPassKind { Float { offset_from_start : Size , ty : Reg } , Integer { offset_from_start : Size , ty : Reg } , Unknown , }
    };
}

RegPassKind!();