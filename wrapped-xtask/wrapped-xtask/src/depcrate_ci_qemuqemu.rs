// Generated macro for Qemu (struct)
macro_rules! Depcrate_ci_qemuQemu {
() => {
// Module: crate::ci::qemu
// Provides: {"Qemu"}
// Dependencies: {}
# [doc = " Run image on QEMU."] # [derive (Args)] pub struct Qemu { # [doc = " Enable hardware acceleration."] # [arg (long)] accel : bool , # [doc = " Run QEMU using `sudo`."] # [arg (long)] sudo : bool , # [doc = " Enable the `microvm` machine type."] # [arg (long)] microvm : bool , # [doc = " Enable PCIe support."] # [arg (long)] pci_e : bool , # [doc = " Enable UEFI."] # [arg (long)] uefi : bool , # [doc = " Devices to enable."] # [arg (long)] devices : Vec < Device > , # [doc = " Do not activate additional virtio features."] # [arg (long)] no_default_virtio_features : bool , # [doc = " Use a TAP device for networking."] # [arg (long)] tap : bool , }
};
}
