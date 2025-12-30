use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: domain_to_ascii_strict");
# [doc = " The [domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii) algorithm,"] # [doc = " with the `beStrict` flag set."] # [doc = ""] # [doc = " Note that this rejects various real-world names including:"] # [doc = " * YouTube CDN nodes"] # [doc = " * Some GitHub user pages"] # [doc = " * Pseudo-hosts used by various TXT record-based protocols."] pub fn domain_to_ascii_strict (domain : & str) -> Result < String , Errors > { Uts46 :: new () . to_ascii (domain . as_bytes () , uts46 :: AsciiDenyList :: STD3 , uts46 :: Hyphens :: Check , uts46 :: DnsLength :: Verify ,) . map (| cow | cow . into_owned ()) }
}