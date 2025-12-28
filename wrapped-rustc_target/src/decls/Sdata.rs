macro_rules! Sdata {
    () => {
        # [derive (Clone , Debug)] struct Sdata { pub prefix : [Option < Reg > ; 8] , pub prefix_index : usize , pub last_offset : Size , pub has_float : bool , pub arg_attribute : ArgAttribute , }
    };
}

Sdata!()