use anyhow::Error;
use crate::frame::Frame;

pub struct CmdSet {
    key: String,
    val: String,
    ttl: Option<u64>,
}

impl CmdSet {
    pub fn from_frame(fm: Frame) -> Result<Self, Error> {
        let args = fm.get_args();
        let mut ttl:Option<u64> = None;
        let t_args = &args[3..];
        if args.len() > 3 {
            for (idx, arg) in t_args.iter().enumerate() {
                match arg.as_str() {
                    "NX" => {
                        if let Some(ttl_val) = t_args.get(idx+1) {
                            ttl = Some(ttl_val.parse::<u64>()? * 1000);
                            break;
                        }
                    },
                    "PX" => {
                        if let Some(ttl_val) = t_args.get(idx+1) {
                            ttl = Some(ttl_val.parse::<u64>()?);
                            break;
                        }
                    },
                    _ => {
                        continue
                    }
                }
            }
        }

        dbg!(ttl);
        Ok(CmdSet{
            key: args[1].clone(), 
            val: args[2].clone(), 
            ttl
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_frame_1() {
        let c = Frame::SimpleString("set".to_string());
        let k = Frame::SimpleString("key1".to_string());
        let v = Frame::SimpleString("msredis".to_string());
        let vec = vec![c, k, v];
        let fm = Frame::Array(vec);
        let res = CmdSet::from_frame(fm);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().key.as_str(), "key1");
        assert_eq!(res.as_ref().unwrap().val.as_str(), "msredis");
    }

    #[test]
    fn test_from_frame_2() {
        let c = Frame::SimpleString("set".to_string());
        let k = Frame::SimpleString("key1".to_string());
        let v = Frame::SimpleString("msredis".to_string());
        let unit = Frame::SimpleString("NX".to_string());
        let t = Frame::SimpleString("20".to_string());
        let vec = vec![c, k, v, unit, t];
        let fm = Frame::Array(vec);
        let res = CmdSet::from_frame(fm);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().key.as_str(), "key1");
        assert_eq!(res.as_ref().unwrap().val.as_str(), "msredis");
        assert_eq!(res.as_ref().unwrap().ttl.is_some(), true);
        assert_eq!(res.as_ref().unwrap().ttl.unwrap(), 20u64 * 1000);
    }
}
