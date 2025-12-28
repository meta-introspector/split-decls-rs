macro_rules! MachineSize {
    () => {
        # [doc = " Represent the size of a component."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub struct MachineSize { num_bits : usize , }
    };
}

MachineSize!()