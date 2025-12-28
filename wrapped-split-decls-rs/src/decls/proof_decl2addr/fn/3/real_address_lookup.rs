use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Get real memory address of a declaration"] fn real_address_lookup (decl_name : & str) -> String { if let Ok (addr) = get_symbol_address (decl_name) { return addr ; } if let Ok (addr) = get_runtime_address (decl_name) { return addr ; } if let Ok (addr) = get_binary_address (decl_name) { return addr ; } format ! ("0x{:x}" , hash_to_addr (decl_name)) }
}