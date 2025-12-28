macro_rules! RegistryId {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub (super) struct RegistryId { addr : usize , }
    };
}

RegistryId!()