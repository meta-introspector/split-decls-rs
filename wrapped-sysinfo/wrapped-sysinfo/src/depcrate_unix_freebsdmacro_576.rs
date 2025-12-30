// Generated macro for macro_576 (macro)
macro_rules! Depcrate_unix_freebsdmacro_576 {
() => {
// Module: crate::unix::freebsd
// Provides: {"macro_576"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "system")] { pub mod cpu ; pub mod motherboard ; pub mod process ; pub mod product ; pub mod system ; pub (crate) use self :: cpu :: CpuInner ; pub (crate) use self :: motherboard :: MotherboardInner ; pub (crate) use self :: process :: ProcessInner ; pub (crate) use self :: product :: ProductInner ; pub (crate) use self :: system :: SystemInner ; pub use self :: system :: { MINIMUM_CPU_UPDATE_INTERVAL , SUPPORTED_SIGNALS } ; } if # [cfg (feature = "disk")] { pub mod disk ; pub (crate) use self :: disk :: DiskInner ; pub (crate) use crate :: unix :: DisksInner ; } if # [cfg (any (feature = "disk" , feature = "system"))] { pub mod ffi ; } if # [cfg (feature = "component")] { pub mod component ; pub (crate) use self :: component :: { ComponentInner , ComponentsInner } ; } if # [cfg (feature = "network")] { pub mod network ; pub (crate) use self :: network :: { NetworkDataInner , NetworksInner } ; } if # [cfg (feature = "user")] { pub (crate) use crate :: unix :: groups :: get_groups ; pub (crate) use crate :: unix :: users :: { get_users , UserInner } ; } }
};
}
