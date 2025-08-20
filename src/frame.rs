use std::{array, f32::consts::E};

type Err = Box<dyn std::error::Error>;


#[derive(Clone)]
pub enum Frame {
    Ok,
    Integer(i64),
    SimpleString(String),
    Array(Vec<Frame>),
    BatchString(String),
    Error(String),
    Null,
}

impl Frame {
    pub fn from_bytes(bytes: &[u8]) -> Result<Frame, Err> {
        match bytes[0] {
            b'+' => Frame::from_simple_string(bytes),
            b'*' => Frame::from_array(bytes),
            _ => Err("Unknown frame type".into()),
        }
    }

    fn from_simple_string(bytes: &[u8]) -> Result<Frame, Err> {
        let end = bytes.iter().position(|&x| x == b'\r').unwrap();
        let content = String::from_utf8(bytes[1..end].to_vec())?;
        Ok(Frame::SimpleString(content))
    }

    fn from_array(bytes: &[u8]) -> Result<Frame, Err> {
        let mut frames = Vec::new();
        let mut start = 0;
        for (i, &item) in bytes.iter().enumerate() {
            if item == b'\r' && i+1 < bytes.len() && bytes[i+1] == b'\n' {
                let part = match std::str::from_utf8(&bytes[start..i]) {
                    Ok(v) => v,
                    Err(_) => return Err("Invalid UTF-8 sequence".into()),
                };

                if !((part.starts_with('*') && part.len()!= 1) || part.starts_with('$')) {
                    frames.push(Frame::SimpleString(part.to_string()));
                }

                start = i + 2;
            }
        }

        Ok(Frame::Array(frames))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Frame::Ok => b"+OK\r\n".to_vec(),
            Frame::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            Frame::Integer(i) => format!(":{}\r\n", i).into_bytes(),
            Frame::Null => b"$-1\r\n".to_vec(),
            Frame::Error(e) => format!("-{}\r\n", e).into_bytes(),
            Frame::Array(arr) => {
                let mut bytes = format!("*{}\r\n", arr.len()).into_bytes();
                for item in arr {
                    bytes.extend(item.as_bytes());
                }
                bytes
            },
            Frame::BatchString(s) => {
                let mut bytes = format!("${}\r\n", s.len()).into_bytes();
                bytes.extend(s.as_bytes());
                bytes.extend(b"\r\n");
                bytes
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Frame::Ok => String::from("ok"),
            Frame::Integer(i) => i.to_string(),
            Frame::SimpleString(ss) => ss.clone(),
            Frame::BatchString(bs) => bs.clone(),
            Frame::Error(e) => e.clone(),
            Frame::Array(array) => {
                let mut res = String::new();
                for item in array {
                    res.push_str(&item.to_string());
                    res.push(' ');
                }
                res.trim_end().to_string()
            },
            Frame::Null => String::new(),
        }
    }

    pub fn get_arg(&self, index: usize) -> Option<String> {
        match self {
            Frame::Array(array) => {
                if index < array.len() {
                    Some(array[index].to_string())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn get_args(&self) -> Vec<String> {
        match self {
            Frame::Array(array) => array.iter().map(|frame| frame.to_string()).collect(),
            _ => Vec::new(),
        }
    }

    pub fn get_args_from_index(&self, start_index: usize) -> Vec<String> {
        match self {
            Frame::Array(array) => {
                if start_index < array.len() {
                    array[start_index..].iter().map(|frame| frame.to_string()).collect()
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new()
        }
    }
}