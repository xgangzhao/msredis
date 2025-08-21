use anyhow::Error;
use crate::frame::Frame;

pub struct CmdGet {
    key: String,
}

impl CmdGet {
    pub fn from_frame(fm: Frame) -> Result<Self, Error> {
        let args = fm.get_args();
        if args.len() != 2 {
            return Err(Error::msg("Invalid argument in command 'get'!"));
        }

        Ok(CmdGet{
            key: args[1].clone(),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_frame() {
        let c = Frame::SimpleString("get".to_string());
        let k = Frame::SimpleString("key1".to_string());
        let vec = vec![c, k];
        let fm = Frame::Array(vec);
        let res = CmdGet::from_frame(fm);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().key.as_str(), "key1");
    }
}
