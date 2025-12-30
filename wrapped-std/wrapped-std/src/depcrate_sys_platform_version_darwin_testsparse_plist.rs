// Generated macro for parse_plist (function)
macro_rules! Depcrate_sys_platform_version_darwin_testsparse_plist {
() => {
// Module: crate::sys::platform_version::darwin::tests
// Provides: {"parse_plist"}
// Dependencies: {}
# [doc = " Test parsing a bunch of different PLists found in the wild, to ensure that"] # [doc = " if we decide to parse it without CoreFoundation in the future, that it"] # [doc = " would continue to work, even on older platforms."] # [test] fn parse_plist () { # [track_caller] fn check ((major , minor , patch) : (u16 , u8 , u8) , ios_version : Option < (u16 , u8 , u8) > , plist : & str ,) { let expected = if cfg ! (target_os = "ios") { if let Some ((ios_major , ios_minor , ios_patch)) = ios_version { pack_os_version (ios_major , ios_minor , ios_patch) } else if cfg ! (target_abi = "macabi") { return ; } else { pack_os_version (major , minor , patch) } } else { pack_os_version (major , minor , patch) } ; let cf_handle = CFHandle :: new () ; assert_eq ! (expected , parse_version_from_plist (& cf_handle , plist . as_bytes ())) ; } let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>ProductBuildVersion</key>
            <string>7B85</string>
            <key>ProductCopyright</key>
            <string>Apple Computer, Inc. 1983-2003</string>
            <key>ProductName</key>
            <string>Mac OS X</string>
            <key>ProductUserVisibleVersion</key>
            <string>10.3</string>
            <key>ProductVersion</key>
            <string>10.3</string>
        </dict>
        </plist>
    "# ; check ((10 , 3 , 0) , None , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>ProductBuildVersion</key>
            <string>11G63</string>
            <key>ProductCopyright</key>
            <string>1983-2012 Apple Inc.</string>
            <key>ProductName</key>
            <string>Mac OS X</string>
            <key>ProductUserVisibleVersion</key>
            <string>10.7.5</string>
            <key>ProductVersion</key>
            <string>10.7.5</string>
        </dict>
        </plist>
    "# ; check ((10 , 7 , 5) , None , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>BuildID</key>
            <string>6A558D8A-E2EA-11EF-A1D3-6222CAA672A8</string>
            <key>ProductBuildVersion</key>
            <string>23H420</string>
            <key>ProductCopyright</key>
            <string>1983-2025 Apple Inc.</string>
            <key>ProductName</key>
            <string>macOS</string>
            <key>ProductUserVisibleVersion</key>
            <string>14.7.4</string>
            <key>ProductVersion</key>
            <string>14.7.4</string>
            <key>iOSSupportVersion</key>
            <string>17.7</string>
        </dict>
        </plist>
    "# ; check ((14 , 7 , 4) , Some ((17 , 7 , 0)) , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>BuildID</key>
            <string>6A558D8A-E2EA-11EF-A1D3-6222CAA672A8</string>
            <key>ProductBuildVersion</key>
            <string>23H420</string>
            <key>ProductCopyright</key>
            <string>1983-2025 Apple Inc.</string>
            <key>ProductName</key>
            <string>Mac OS X</string>
            <key>ProductUserVisibleVersion</key>
            <string>10.16</string>
            <key>ProductVersion</key>
            <string>10.16</string>
            <key>iOSSupportVersion</key>
            <string>17.7</string>
        </dict>
        </plist>
    "# ; check ((10 , 16 , 0) , Some ((17 , 7 , 0)) , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>BuildID</key>
            <string>67A50F62-00DA-11F0-BDB6-F99BB8310D2A</string>
            <key>ProductBuildVersion</key>
            <string>24E5238a</string>
            <key>ProductCopyright</key>
            <string>1983-2025 Apple Inc.</string>
            <key>ProductName</key>
            <string>macOS</string>
            <key>ProductUserVisibleVersion</key>
            <string>15.4</string>
            <key>ProductVersion</key>
            <string>15.4</string>
            <key>iOSSupportVersion</key>
            <string>18.4</string>
        </dict>
        </plist>
    "# ; check ((15 , 4 , 0) , Some ((18 , 4 , 0)) , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>BuildID</key>
            <string>210B8A2C-09C3-11EF-9DB8-273A64AEFA1C</string>
            <key>ProductBuildVersion</key>
            <string>21F79</string>
            <key>ProductCopyright</key>
            <string>1983-2024 Apple Inc.</string>
            <key>ProductName</key>
            <string>iPhone OS</string>
            <key>ProductVersion</key>
            <string>17.5</string>
        </dict>
        </plist>
    "# ; check ((17 , 5 , 0) , None , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>BuildID</key>
            <string>57CEFDE6-D079-11EF-837C-8B8C7961D0AC</string>
            <key>ProductBuildVersion</key>
            <string>22N895</string>
            <key>ProductCopyright</key>
            <string>1983-2025 Apple Inc.</string>
            <key>ProductName</key>
            <string>xrOS</string>
            <key>ProductVersion</key>
            <string>2.3</string>
            <key>SystemImageID</key>
            <string>D332C7F1-08DF-4DD9-8122-94EF39A1FB92</string>
            <key>iOSSupportVersion</key>
            <string>18.3</string>
        </dict>
        </plist>
    "# ; check ((2 , 3 , 0) , Some ((18 , 3 , 0)) , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>BuildID</key>
            <string>617587B0-B059-11EF-BE70-4380EDE44645</string>
            <key>ProductBuildVersion</key>
            <string>22K154</string>
            <key>ProductCopyright</key>
            <string>1983-2024 Apple Inc.</string>
            <key>ProductName</key>
            <string>Apple TVOS</string>
            <key>ProductVersion</key>
            <string>18.2</string>
            <key>SystemImageID</key>
            <string>8BB5A425-33F0-4821-9F93-40E7ED92F4E0</string>
        </dict>
        </plist>
    "# ; check ((18 , 2 , 0) , None , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>BuildID</key>
            <string>BAAE2D54-B122-11EF-BF78-C6C6836B724A</string>
            <key>ProductBuildVersion</key>
            <string>22S99</string>
            <key>ProductCopyright</key>
            <string>1983-2024 Apple Inc.</string>
            <key>ProductName</key>
            <string>Watch OS</string>
            <key>ProductVersion</key>
            <string>11.2</string>
            <key>SystemImageID</key>
            <string>79F773E2-2041-43B4-98EE-FAE52402AE95</string>
        </dict>
        </plist>
    "# ; check ((11 , 2 , 0) , None , plist) ; let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
            <key>ProductBuildVersion</key>
            <string>13G37</string>
            <key>ProductCopyright</key>
            <string>1983-2019 Apple Inc.</string>
            <key>ProductName</key>
            <string>iPhone OS</string>
            <key>ProductVersion</key>
            <string>9.3.6</string>
        </dict>
        </plist>
    "# ; check ((9 , 3 , 6) , None , plist) ; }
};
}
