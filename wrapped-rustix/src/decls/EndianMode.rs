macro_rules! EndianMode {
    () => {
        # [doc = " `PR_ENDIAN_*` values for use with [`endian_mode`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] pub enum EndianMode { # [doc = " Big endian mode."] Big = PR_ENDIAN_BIG , # [doc = " True little endian mode."] Little = PR_ENDIAN_LITTLE , # [doc = " `PowerPC` pseudo little endian."] PowerPCLittle = PR_ENDIAN_PPC_LITTLE , }
    };
}

EndianMode!()