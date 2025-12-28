macro_rules! deps {
    () => {
        MachineInfo!();
        Allocation!();
        Error!();
    };
}

macro_rules! impl_413 {
    () => {
        deps!();
        impl Allocation { # [doc = " Get a vector of bytes for an Allocation that has been fully initialized"] pub fn raw_bytes (& self) -> Result < Vec < u8 > , Error > { self . bytes . iter () . copied () . collect :: < Option < Vec < _ > > > () . ok_or_else (| | error ! ("Found uninitialized bytes: `{:?}`" , self . bytes)) } # [doc = " Read a uint value from the specified range."] pub fn read_partial_uint (& self , range : Range < usize >) -> Result < u128 , Error > { if range . end - range . start > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } if range . end > self . bytes . len () { return Err (error ! ("Range is out of bounds. Allocation length is `{}`, but requested range `{:?}`" , self . bytes . len () , range)) ; } let raw = self . bytes [range] . iter () . copied () . collect :: < Option < Vec < _ > > > () . ok_or_else (| | error ! ("Found uninitialized bytes: `{:?}`" , self . bytes)) ? ; read_target_uint (& raw) } # [doc = " Read this allocation and try to convert it to an unassigned integer."] pub fn read_uint (& self) -> Result < u128 , Error > { if self . bytes . len () > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } let raw = self . raw_bytes () ? ; read_target_uint (& raw) } # [doc = " Read this allocation and try to convert it to a signed integer."] pub fn read_int (& self) -> Result < i128 , Error > { if self . bytes . len () > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } let raw = self . raw_bytes () ? ; read_target_int (& raw) } # [doc = " Read this allocation and try to convert it to a boolean."] pub fn read_bool (& self) -> Result < bool , Error > { match self . read_int () ? { 0 => Ok (false) , 1 => Ok (true) , val => Err (error ! ("Unexpected value for bool: `{val}`")) , } } # [doc = " Read this allocation as a pointer and return whether it represents a `null` pointer."] pub fn is_null (& self) -> Result < bool , Error > { let len = self . bytes . len () ; let ptr_len = MachineInfo :: target_pointer_width () . bytes () ; if len != ptr_len { return Err (error ! ("Expected width of pointer (`{ptr_len}`), but found: `{len}`")) ; } Ok (self . read_uint () ? == 0 && self . provenance . ptrs . is_empty ()) } }
    };
}

impl_413!()