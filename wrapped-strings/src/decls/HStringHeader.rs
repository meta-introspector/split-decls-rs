macro_rules! deps {
    () => {
        RefCount!();
    };
}

macro_rules! HStringHeader {
    () => {
        deps!();
        # [repr (C)] pub struct HStringHeader { pub flags : u32 , pub len : u32 , pub _0 : u32 , pub _1 : u32 , pub data : * mut u16 , pub count : RefCount , pub buffer_start : u16 , }
    };
}

HStringHeader!();