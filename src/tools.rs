use anyhow::Context;
use log::error;

// Handle anyhow errors - log them and add context.
macro_rules! handle_err {
    ($e:expr, $s:expr) => {
        $e.map_err(|e| { error!("{}: {:?}", $s, e); e }).context($s)
    };
}

macro_rules! log_err {
    ($e:expr) => {
        $e.map_err(|e| {error!("{:?}", e); e})
    }
}

pub(super) use handle_err;
pub(super) use log_err;
