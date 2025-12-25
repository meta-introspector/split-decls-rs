use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn invalid_data() -> Error {
    Error::from_hresult(WIN32_ERROR(ERROR_INVALID_DATA).to_hresult())
}
