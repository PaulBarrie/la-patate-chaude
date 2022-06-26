use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Welcome {
    version: u8
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Hello,
    Welcome(Welcome)
}
