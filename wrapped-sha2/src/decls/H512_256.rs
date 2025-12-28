macro_rules! deps {
    () => {
        State512!();
    };
}

macro_rules! H512_256 {
    () => {
        deps!();
        pub (crate) const H512_256 : State512 = [0x22312194fc2bf72c , 0x9f555fa3c84c64c2 , 0x2393b86b6f53b151 , 0x963877195940eabd , 0x96283ee2a88effe3 , 0xbe5e1e2553863992 , 0x2b0199fc2c85b8aa , 0x0eb72ddc81c52ca2 ,] ;
    };
}

H512_256!()