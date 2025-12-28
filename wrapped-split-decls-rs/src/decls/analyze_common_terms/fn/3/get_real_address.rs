use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Get real memory address using multiple methods"] fn get_real_address (symbol : & str) -> String { if let Ok (addr) = get_nm_address (symbol) { return addr ; } if let Ok (addr) = get_objdump_address (symbol) { return addr ; } if let Ok (addr) = get_rust_symbol_address (symbol) { return addr ; } format ! ("0x{:x}(hash)" , hash_symbol (symbol)) }
}