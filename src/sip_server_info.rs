use crate::credential::*;

#[derive(Debug, Clone)]
pub struct SipServerInfo {
    ip_tel_num: u32,
    username: String,
    password: String,
    rtp_port: [Option<u16>; 2],
    dscp: u32,
    sip_server_addr: String,
    sip_server_port: u16,
    register_server_addr: String,
    register_server_port: u16,
    sip_domain: String,
}

impl SipServerInfo {
    pub fn ip_tel_number(&self) -> u32 {
        return self.ip_tel_num;
    }
    pub fn credentials(&self) -> Credential {
        Credential::id_password(self.username.clone(), self.password.clone())
    }
    pub fn rtp_port(&self) -> [Option<u16>; 2] {
        self.rtp_port
    }
}
use std::convert::TryFrom;
impl TryFrom<String> for SipServerInfo {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        // 改行で分割する
        let ss = s.replace("\r", "").replace("\n\n", "");
        let ss: Vec<&str> = ss.split("\n").collect();
        let code_s: Vec<&str> = ss[0].split("=").collect();
        let code: u32 = if code_s[0] == "ResultCode" {
            code_s[1].parse().unwrap()
        } else {
            return Err(format!("non compatible format"));
        };
        if ss.len() < 12 || code != 0 {
            return Err(format!("Unknown Error : {}", code_s[1]));
        }
        use std::collections::HashMap;
        let mut kv_list = HashMap::new();
        for kv in ss {
            let kv: Vec<&str> = kv.split("=").collect();
            kv_list.insert(kv[0], kv[1]);
        }
        let set_str = |s: &str| {
            if s == "" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        };
        Ok(SipServerInfo {
            ip_tel_num: kv_list["ip_tel_num"].parse().unwrap(),
            username: kv_list["username"].to_owned(),
            password: kv_list["password"].to_owned(),
            rtp_port: [set_str(kv_list["rtpport1"]), set_str(kv_list["rtpport2"])],
            dscp: kv_list["dscp"].parse().unwrap(),
            sip_server_addr: kv_list["sipsv_addr"].to_owned(),
            sip_server_port: kv_list["sipsv_port"].parse().unwrap(),
            register_server_addr: kv_list["regsv_addr"].to_owned(),
            register_server_port: kv_list["regsv_port"].parse().unwrap(),
            sip_domain: kv_list["sip_domain"].to_owned(),
        })
    }
}
