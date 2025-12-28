macro_rules! assert_elapsed {
    () => {
        # [doc = " Asserts that an exact duration has elapsed since the start instant ±1ms."] # [doc = ""] # [doc = " ```rust"] # [doc = " use tokio::time::{self, Instant};"] # [doc = " use std::time::Duration;"] # [doc = " use tokio_test::assert_elapsed;"] # [doc = " # async fn test_time_passed() {"] # [doc = ""] # [doc = " let start = Instant::now();"] # [doc = " let dur = Duration::from_millis(50);"] # [doc = " time::sleep(dur).await;"] # [doc = " assert_elapsed!(start, dur);"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " This 1ms buffer is required because Tokio's hashed-wheel timer has finite time resolution and"] # [doc = " will not always sleep for the exact interval."] # [macro_export] macro_rules ! assert_elapsed { ($ start : expr , $ dur : expr) => { { let elapsed = $ start . elapsed () ; let lower : std :: time :: Duration = $ dur ; assert ! (elapsed >= lower && elapsed <= lower + std :: time :: Duration :: from_millis (1) , "actual = {:?}, expected = {:?}" , elapsed , lower) ; } } ; }
    };
}

assert_elapsed!()