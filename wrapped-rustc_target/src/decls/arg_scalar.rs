macro_rules! deps {
    () => {
        Sdata!();
    };
}

macro_rules! arg_scalar {
    () => {
        deps!();
        fn arg_scalar < C > (cx : & C , scalar : & Scalar , offset : Size , mut data : Sdata) -> Sdata where C : HasDataLayout , { let dl = cx . data_layout () ; if ! matches ! (scalar . primitive () , Primitive :: Float (Float :: F32 | Float :: F64)) { return data ; } data . has_float = true ; if ! data . last_offset . is_aligned (dl . f64_align . abi) && data . last_offset < offset { if data . prefix_index == data . prefix . len () { return data ; } data . prefix [data . prefix_index] = Some (Reg :: i32 ()) ; data . prefix_index += 1 ; data . last_offset = data . last_offset + Reg :: i32 () . size ; } for _ in 0 .. ((offset - data . last_offset) . bits () / 64) . min ((data . prefix . len () - data . prefix_index) as u64) { data . prefix [data . prefix_index] = Some (Reg :: i64 ()) ; data . prefix_index += 1 ; data . last_offset = data . last_offset + Reg :: i64 () . size ; } if data . last_offset < offset { if data . prefix_index == data . prefix . len () { return data ; } data . prefix [data . prefix_index] = Some (Reg :: i32 ()) ; data . prefix_index += 1 ; data . last_offset = data . last_offset + Reg :: i32 () . size ; } if data . prefix_index == data . prefix . len () { return data ; } if scalar . primitive () == Primitive :: Float (Float :: F32) { data . arg_attribute = ArgAttribute :: InReg ; data . prefix [data . prefix_index] = Some (Reg :: f32 ()) ; data . last_offset = offset + Reg :: f32 () . size ; } else { data . prefix [data . prefix_index] = Some (Reg :: f64 ()) ; data . last_offset = offset + Reg :: f64 () . size ; } data . prefix_index += 1 ; data }
    };
}

arg_scalar!()