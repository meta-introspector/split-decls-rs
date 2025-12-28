macro_rules! deps {
    () => {
        Storage!();
    };
}

macro_rules! DatabaseImpl {
    () => {
        deps!();
        # [doc = " Default database implementation that you can use if you don't"] # [doc = " require any custom user data."] # [derive (Clone)] pub struct DatabaseImpl { storage : Storage < Self > , }
    };
}

DatabaseImpl!();