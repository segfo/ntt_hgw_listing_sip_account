pub use ipconfig;
mod credential;
mod nic_info;
use nic_info::*;
mod sip_server_info;
use sip_server_info::*;
mod common;

const LIST: &str = "cas_tel_list";
const CONF: &str = "cas_tel_conf";

async fn get_sip_list(hgw: &str) -> (u32, Vec<String>) {
    let mut list_res = surf::get(&format!("http://{}/{}/", hgw, LIST))
        .await
        .unwrap();
    let body = list_res.body_string().await.unwrap();
    let raw_body = body.clone();
    // 改行で分割する
    let body = body.replace("\r", "").replace("\n\n", "");
    let body: Vec<&str> = body.split("\n").collect();
    // key = value形式で入ってくるので分割する
    let code: Vec<&str> = body[0].split("=").collect();
    let code: u32 = if code[0] == "ResultCode" {
        code[1].parse().unwrap()
    } else {
        u32::MAX // non compatible format
    };
    let mut ret_list = Vec::new();
    if code == 0 {
        // codeが0の場合成功ステータスなので、2行目にデータが入ってくる。
        // key = value(配列)　形式で入ってくるので更に分割する。
        let list: Vec<&str> = body[1].split("=").collect();
        let list: Vec<&str> = list[1].split(",").collect();
        for s in list {
            ret_list.push(s.to_owned());
        }
    } else {
        // エラーコードの場合はリストの中にAPIレスポンスをそのまま突っ込む
        ret_list.push(raw_body);
    }
    (code, ret_list)
}

use std::collections::HashMap;
use std::convert::TryFrom;
async fn get_sip_password(hgw: &str, id: &str) -> Result<HashMap<u32, SipServerInfo>, String> {
    use surf::http::{mime::JSON, Method, Url};
    let (code, list) = get_sip_list(hgw).await;
    if code != 0 {
        return Err(format!(
            "内線番号の取得に失敗しました。\n[APIレスポンス]\n{}",
            list[0]
        ));
    }
    let mut map: HashMap<u32, SipServerInfo> = HashMap::new();
    for number in list {
        let number = number.parse::<u32>().unwrap();
        let config_request = format!("id={}&ip_tel_num={}&term_type=1}}", id, number);
        let config_res = surf::post(&format!("http://{}/{}/", hgw, CONF))
            .body(config_request)
            .content_type(JSON)
            .recv_string()
            .await
            .unwrap();
        match SipServerInfo::try_from(config_res) {
            Ok(ssi) => {
                map.insert(ssi.ip_tel_number(), ssi);
            }
            Err(e) => {
                println!("API Response parse error : {}", e);
            }
        };
    }
    return Ok(map);
}

#[tokio::main]
async fn main() {
    let hgw = "192.168.1.1";

    let mut nic_list = Vec::new();
    for adapter in ipconfig::get_adapters().unwrap() {
        // デフォルトゲートウェイが設定されているアダプタを取得する。
        if adapter.gateways().len() != 0 {
            nic_list.push(NetworkInterfaceInfo::from(adapter));
        }
    }
    use std::collections::BTreeMap;
    let mut btmap = BTreeMap::new();

    if nic_list.len() > 0 {
        let mac_addr_str = nic_list[0].mac_addr_string();
        if mac_addr_str.is_some() {
            let mac_addr = mac_addr_str.unwrap().replace(":", "");
            // [問い合わせを2回投げる理由]
            // 登録されているかどうかを問い合わせるとそのアカウントが使用中となるため
            // 1/2/3が有ったとして、1回目では　1/2　しか取れず、3の情報が取れない
            // 3の情報を取るためには、使用中の3に対して再度クエリを掛ける必要がある。
            // そのため2回問い合わせている。
            for i in 0..2{
                let mut map = get_sip_password(hgw, &mac_addr).await.unwrap();
                for (k, v) in map.iter() {
                    btmap.insert(*k, v.clone());
                }
            }
        }
    }
    for (k, v) in btmap.iter() {
        println!("内線番号: {}", v.ip_tel_number());
        println!("認証情報: {}", v.credentials());
    }
}
