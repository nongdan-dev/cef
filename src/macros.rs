#[macro_export]
macro_rules! println_debug {
    ($($v:tt)*) => {
        #[cfg(feature = "debug")]
        println!($($v)*);
    };
}

#[macro_export]
macro_rules! not_rebuild {
    ($($v:tt)*) => {
        #[cfg(not(feature = "rebuild"))]
        $($v)*;
    };
}
