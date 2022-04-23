// Log error with description.
macro_rules! log_err_desc {
    ($e:expr, $s:expr) => {
        if let Err(e) = $e {
            error!("{} {}", $s, e)
        }
    };
}

// Log error with description and return error.
macro_rules! log_err_desc_ret {
    ($e:expr, $s:expr) => {
        $e.map_err(|e| { error!("{}: {:?}", $s, e); e})
    };
}

// Log error and return error.
macro_rules! log_err_ret {
    ($e:expr) => {
        $e.map_err(|e| {error!("{:?}", e); e})
    }
}

pub(super) use log_err_desc;
pub(super) use log_err_desc_ret;
pub(super) use log_err_ret;
