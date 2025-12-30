// Generated macro for Device (enum)
macro_rules! Depcrate_ci_qemuDevice {
() => {
// Module: crate::ci::qemu
// Provides: {"Device"}
// Dependencies: {}
# [derive (ValueEnum , PartialEq , Eq , Clone , Copy)] pub enum Device { # [doc = " Cadence Gigabit Ethernet MAC (GEM)."] CadenceGem , # [doc = " RTL8139."] Rtl8139 , # [doc = " virtio-console via MMIO."] VirtioConsoleMmio , # [doc = " virtio-console via PCI."] VirtioConsolePci , # [doc = " virtio-fs via PCI."] # [doc = ""] # [doc = " This option also starts the `virtiofsd` virtio-fs vhost-user device daemon."] VirtioFsPci , # [doc = " virtio-net via MMIO."] VirtioNetMmio , # [doc = " virtio-net via PCI."] VirtioNetPci , }
};
}
