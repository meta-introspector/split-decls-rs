macro_rules! macro_35 {
    () => {
        decl_derive ! { [TryFromU32] => # [doc = " Derives `TryFrom<u32>` for the annotated `enum`, which must have no fields."] # [doc = " Each variant maps to the value it would produce under an `as u32` cast."] # [doc = ""] # [doc = " The error type is `u32`."] try_from :: try_from_u32 }
    };
}

macro_35!()