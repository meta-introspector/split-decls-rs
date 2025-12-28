use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " RDF URL Blob - Complete system state as a URL that can be piped as stdin"] # [derive (Debug , Clone)] pub struct RdfUrlBlob { # [doc = " RDF/Turtle representation of system state"] pub rdf_content : String , # [doc = " Base64 encoded blob for URL transport"] pub url_blob : String , # [doc = " Metadata about the state"] pub metadata : BlobMetadata , }