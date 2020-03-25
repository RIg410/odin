#[macro_export]
macro_rules! log_error {
    ($dst:expr) => {
        if let Err(err) = $dst {
            error!("{} -> {}", stringify!($dst), err);
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($dst:expr) => {
        if let Err(err) = $dst {
            warn!("{} -> {}", stringify!($dst), err);
        }
    };
}
