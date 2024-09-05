#[macro_export]
macro_rules! unwrap_or_log {
    ($result:expr, $default:expr) => {
        match $result {
            Ok(val) => val,
            Err(e) => {
                #[cfg(feature = "log")]
                log::error!("Error: {:?}", e);
                #[cfg(feature = "tracing")]
                tracing::error!("Error: {:?}", e);
                $default
            }
        }
    };
}

#[macro_export]
macro_rules! expect_or_log {
    ($result:expr, $msg:expr) => {
        match $result {
            Ok(val) => val,
            Err(e) => {
                #[cfg(feature = "log")]
                log::error!("{}: {:?}", $msg, e);
                #[cfg(feature = "tracing")]
                tracing::error!("{}: {:?}", $msg, e);
                panic!($msg);
            }
        }
    };
}
