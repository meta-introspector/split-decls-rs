macro_rules! deps {
    () => {
        HexCase!();
    };
}

macro_rules! Radix {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] enum Radix { Decimal , Hexadecimal { case : HexCase } , Octal , Binary , }
    };
}

Radix!()