#[macro_export]
macro_rules! main_tokio {
    () => {
        // tokio main here
    };
    ( $( $x:expr ),+ ) => {{
        $x
        // tokio main again
    }};
}

#[cfg(feature = "tokio")]
#[cfg_attr(feature = "tokio", macro_export)]
macro_rules! main_runtime {
    () => {
        // tokio runtime here
    };
    ( $( $x:expr ),+ ) => {{
        $x
        // tokio runtime again
    }};
}
