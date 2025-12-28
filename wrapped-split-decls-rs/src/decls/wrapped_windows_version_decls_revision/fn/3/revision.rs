use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Gets the revision number of the operating system."] pub fn revision () -> u32 { let mut value = [0 ; 4] ; let mut len = 4 ; let result = unsafe { RegGetValueA (HKEY_LOCAL_MACHINE , b"SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\0" . as_ptr () , b"UBR\0" . as_ptr () , RRF_RT_REG_DWORD , core :: ptr :: null_mut () , value . as_mut_ptr () as _ , & mut len ,) } ; if result == 0 { u32 :: from_le_bytes (value) } else { 0 } }