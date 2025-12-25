use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Writes a Windows resource script file for the rust compiler with the product and file version information
/// into `rc_path`
fn write_resource_script_file(
    rc_path: &path::Path,
    file_description: &str,
    filetype: VersionInfoFileType,
) {
    let mut resource_script = RESOURCE_TEMPLATE.to_string();
    let descriptive_version = env::var("CFG_VERSION").unwrap_or("unknown".to_string());
    let product_name = product_name(env::var("CFG_RELEASE_CHANNEL").unwrap());
    let cfg_release = env::var("CFG_RELEASE").unwrap();
    let version = parse_version(cfg_release.split("-").next().unwrap_or("0.0.0"))
        .expect("valid CFG_RELEASE version");
    resource_script = resource_script
        .replace("@RUSTC_FILEDESCRIPTION_STR@", file_description)
        .replace("@RUSTC_FILETYPE@", &format!("{}", filetype as u32))
        .replace("@RUSTC_FILEVERSION_QUAD@", &version.to_quad_string())
        .replace("@RUSTC_FILEVERSION_STR@", &descriptive_version)
        .replace("@RUSTC_PRODUCTNAME_STR@", &product_name)
        .replace("@RUSTC_PRODUCTVERSION_QUAD@", &version.to_quad_string())
        .replace("@RUSTC_PRODUCTVERSION_STR@", &descriptive_version);
    fs::write(&rc_path, resource_script)
        .unwrap_or_else(|_| panic!("failed to write resource file {}", rc_path.display()));
}
