macro_rules! deps {
    () => {
        SingleCache!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < V > Default for SingleCache < V > { fn default () -> Self { SingleCache { cache : OnceLock :: new () } } }
    };
}

impl_194!()