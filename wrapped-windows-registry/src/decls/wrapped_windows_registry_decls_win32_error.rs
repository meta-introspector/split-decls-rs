use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn win32_error(result: u32) -> Result<()> {
    if result == 0 {
        Ok(())
    } else {
        Err(Error::from_hresult(WIN32_ERROR(result).to_hresult()))
    }
}
