macro_rules! deps {
    () => {
        InitKind!();
        InitLocation!();
    };
}

macro_rules! Init {
    () => {
        deps!();
        # [doc = " `Init` represents a point in a program that initializes some L-value;"] # [derive (Copy , Clone)] pub struct Init { # [doc = " path being initialized"] pub path : MovePathIndex , # [doc = " location of initialization"] pub location : InitLocation , # [doc = " Extra information about this initialization"] pub kind : InitKind , }
    };
}

Init!()