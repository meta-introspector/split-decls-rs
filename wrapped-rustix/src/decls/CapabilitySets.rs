macro_rules! CapabilitySets {
    () => {
        # [doc = " `__user_cap_data_struct`"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct CapabilitySets { # [doc = " `__user_cap_data_struct.effective`"] pub effective : CapabilitySet , # [doc = " `__user_cap_data_struct.permitted`"] pub permitted : CapabilitySet , # [doc = " `__user_cap_data_struct.inheritable`"] pub inheritable : CapabilitySet , }
    };
}

CapabilitySets!();