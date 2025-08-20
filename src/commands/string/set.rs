use anyhow::Error;
use crate::frame::Frame;

pub struct CmdSet {
    key: String,
    val: String,
    ttl: Option<u64>,
}

impl CmdSet {
    pub fn from_frame(fm: Frame) -> Result(Self, Error) {
        let key = fm.get_arg(1);
        let val = fm.get_arg(2);
        if key.is_none() || val.is_none() {
            return Err(Error::msg("Failed to parse arguments for 'set' command!"));
        }
        key = key.unwrap();
    }
}