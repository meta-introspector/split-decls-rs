// Generated macro for impl_245 (impl)
macro_rules! Depcrate_debugimpl_245 {
() => {
// Module: crate::debug
// Provides: {"impl_245"}
// Dependencies: {}
# [cfg (feature = "disk")] impl std :: fmt :: Debug for crate :: Disk { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (fmt , "Disk({:?})[FS: {:?}][Type: {:?}][removable: {}][I/O: {:?}] mounted on {:?}: {}/{} B" , self . name () , self . file_system () , self . kind () , if self . is_removable () { "yes" } else { "no" } , self . usage () , self . mount_point () , self . available_space () , self . total_space () ,) } }
};
}
