// Generated macro for TryLockError (enum)
macro_rules! Depcrate_fsTryLockError {
() => {
// Module: crate::fs
// Provides: {"TryLockError"}
// Dependencies: {}
# [doc = " An enumeration of possible errors which can occur while trying to acquire a lock"] # [doc = " from the [`try_lock`] method and [`try_lock_shared`] method on a [`File`]."] # [doc = ""] # [doc = " [`try_lock`]: File::try_lock"] # [doc = " [`try_lock_shared`]: File::try_lock_shared"] # [stable (feature = "file_lock" , since = "1.89.0")] pub enum TryLockError { # [doc = " The lock could not be acquired due to an I/O error on the file. The standard library will"] # [doc = " not return an [`ErrorKind::WouldBlock`] error inside [`TryLockError::Error`]"] # [doc = ""] # [doc = " [`ErrorKind::WouldBlock`]: io::ErrorKind::WouldBlock"] Error (io :: Error) , # [doc = " The lock could not be acquired at this time because it is held by another handle/process."] WouldBlock , }
};
}
