macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        impl Target { pub fn parse_data_layout (& self) -> Result < TargetDataLayout , TargetDataLayoutErrors < '_ > > { let mut dl = TargetDataLayout :: parse_from_llvm_datalayout_string (& self . data_layout , self . options . default_address_space ,) ? ; if dl . endian != self . endian { return Err (TargetDataLayoutErrors :: InconsistentTargetArchitecture { dl : dl . endian . as_str () , target : self . endian . as_str () , }) ; } let target_pointer_width : u64 = self . pointer_width . into () ; let dl_pointer_size : u64 = dl . pointer_size () . bits () ; if dl_pointer_size != target_pointer_width { return Err (TargetDataLayoutErrors :: InconsistentTargetPointerWidth { pointer_size : dl_pointer_size , target : self . pointer_width , }) ; } dl . c_enum_min_size = Integer :: from_size (Size :: from_bits (self . c_enum_min_bits . unwrap_or (self . c_int_width as _) ,)) . map_err (| err | TargetDataLayoutErrors :: InvalidBitsSize { err }) ? ; Ok (dl) } }
    };
}

impl_530!();