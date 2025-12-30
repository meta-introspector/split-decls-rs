// Generated macro for InfoType (enum)
macro_rules! Depcrate_unix_linux_systemInfoType {
() => {
// Module: crate::unix::linux::system
// Provides: {"InfoType"}
// Dependencies: {}
# [derive (PartialEq , Eq)] enum InfoType { # [doc = " The end-user friendly name of:"] # [doc = " - Android: The device model"] # [doc = " - Linux: The distributions name"] Name , OsVersion , # [doc = " Machine-parseable ID of a distribution, see"] # [doc = " https://www.freedesktop.org/software/systemd/man/os-release.html#ID="] DistributionID , # [doc = " Machine-parseable ID_LIKE of related distributions, see"] # [doc = " <https://www.freedesktop.org/software/systemd/man/latest/os-release.html#ID_LIKE=>"] DistributionIDLike , }
};
}
