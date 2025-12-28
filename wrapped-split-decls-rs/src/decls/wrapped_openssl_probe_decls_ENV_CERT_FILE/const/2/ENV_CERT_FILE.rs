use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " The OpenSSL environment variable to configure what certificate file to use."] pub const ENV_CERT_FILE : & 'static str = "SSL_CERT_FILE" ;
}