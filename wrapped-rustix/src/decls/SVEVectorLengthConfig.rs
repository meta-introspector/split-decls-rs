macro_rules! SVEVectorLengthConfig {
    () => {
        # [doc = " Scalable Vector Extension vector length configuration."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct SVEVectorLengthConfig { # [doc = " Vector length in bytes."] pub vector_length_in_bytes : u32 , # [doc = " Vector length inherited across `execve`."] pub vector_length_inherited_across_execve : bool , }
    };
}

SVEVectorLengthConfig!();