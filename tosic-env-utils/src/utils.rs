#[macro_export]
macro_rules! env {
    // Branch for string case, with a default value
    ($key:expr, $default:expr) => {{
        match std::env::var($key) {
            Ok(val) => val,
            Err(_) => {
                #[cfg(feature = "log")]
                log::info!("Environment variable {} not set, using default: {}", $key, $default);
                #[cfg(feature = "tracing")]
                tracing::info!("Environment variable {} not set, using default: {}", $key, $default);
                $default.to_string()
            }
        }
    }};

    // Branch for string case, required variable (panic if not set)
    ($key:expr) => {{
        match std::env::var($key) {
            Ok(val) => val,
            Err(_) => {
                #[cfg(feature = "log")]
                log::error!("Environment variable {} is not set. Terminating program.", $key);
                #[cfg(feature = "tracing")]
                tracing::error!("Environment variable {} is not set. Terminating program.", $key);
                panic!("Environment variable {} is required but not set.", $key);
            }
        }
    }};

    // Branch for type conversion case, with a default value
    ($key:expr, $default:expr, $type:ty) => {{
        match std::env::var($key) {
            Ok(val) => match <$type>::from_str(&val) {
                Ok(parsed_val) => parsed_val,
                Err(_) => {
                    #[cfg(feature = "log")]
                    log::warn!("Failed to parse environment variable {}: '{}'. Using default: {}", $key, val, $default);
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Failed to parse environment variable {}: '{}'. Using default: {}", $key, val, $default);
                    $default
                }
            },
            Err(_) => {
                #[cfg(feature = "log")]
                log::info!("Environment variable {} not set, using default: {}", $key, $default);
                #[cfg(feature = "tracing")]
                tracing::info!("Environment variable {} not set, using default: {}", $key, $default);
                $default
            }
        }
    }};

    // Branch for type conversion case, required variable (panic if not set or cannot parse)
    ($key:expr, $type:ty) => {{
        match std::env::var($key) {
            Ok(val) => match <$type>::from_str(&val) {
                Ok(parsed_val) => parsed_val,
                Err(_) => {
                    #[cfg(feature = "log")]
                    log::error!("Failed to parse environment variable {}: '{}'. Expected type: {}. Terminating program.", $key, val, stringify!($type));
                    #[cfg(feature = "tracing")]
                    tracing::error!("Failed to parse environment variable {}: '{}'. Expected type: {}. Terminating program.", $key, val, stringify!($type));
                    panic!("Environment variable {} is set but cannot be parsed as type {}.", $key, stringify!($type));
                }
            },
            Err(_) => {
                #[cfg(feature = "log")]
                log::error!("Environment variable {} is not set. Terminating program.", $key);
                #[cfg(feature = "tracing")]
                tracing::error!("Environment variable {} is not set. Terminating program.", $key);
                panic!("Environment variable {} is required but not set.", $key);
            }
        }
    }};
}
