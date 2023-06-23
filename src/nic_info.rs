use crate::common::*;
use std::net::IpAddr;
pub struct NetworkInterfaceInfo {
    description: String,
    ip_addr: Vec<IpAddr>,
    mac_addr: Option<[u8; 6]>,
    gateway: Vec<IpAddr>,
    dns_server: Vec<IpAddr>,
    tx_link_speed_bps: usize,
    rx_link_speed_bps: usize,
}

impl NetworkInterfaceInfo {
    pub fn mac_addr_string(&self) -> Option<String> {
        if self.mac_addr.is_some() {
            let m = self.mac_addr.unwrap();
            Some(format!(
                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                m[0], m[1], m[2], m[3], m[4], m[5]
            ))
        } else {
            None
        }
    }
    pub fn tx_link_speed_bps_string(&self) -> String {
        NetworkInterfaceInfo::link_speed_bps_string(self.tx_link_speed_bps)
    }
    pub fn rx_link_speed_bps_string(&self) -> String {
        NetworkInterfaceInfo::link_speed_bps_string(self.rx_link_speed_bps)
    }
    fn link_speed_bps_string(speed: usize) -> String {
        format!("{}Bits / sec", to_human_readable(speed))
    }
}

impl From<ipconfig::Adapter> for NetworkInterfaceInfo {
    fn from(adapter: ipconfig::Adapter) -> Self {
        let m = adapter.physical_address();
        let mac_addr = if m.is_some() {
            let m = m.unwrap();
            Some([m[0], m[1], m[2], m[3], m[4], m[5]])
        } else {
            None
        };
        NetworkInterfaceInfo {
            description: adapter.description().to_owned(),
            ip_addr: adapter.ip_addresses().to_vec(),
            mac_addr: mac_addr,
            gateway: adapter.gateways().to_vec(),
            dns_server: adapter.dns_servers().to_vec(),
            tx_link_speed_bps: adapter.transmit_link_speed() as usize,
            rx_link_speed_bps: adapter.receive_link_speed() as usize,
        }
    }
}

pub fn show_nic(adapter: &NetworkInterfaceInfo) {
    println!("[{}]", adapter.description);
    {
        println!("\tIP Address");
        for ip in &adapter.ip_addr {
            println!("\t\t{}", ip);
        }
    }
    {
        println!("\tMAC Address");
        if adapter.mac_addr.is_some() {
            print!("\t\t");
            let mut cnt = 0;
            for m in &adapter.mac_addr {
                println!(
                    "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                    m[0], m[1], m[2], m[3], m[4], m[5]
                );
            }
        } else {
            println!("\t\t MAC Address が未割り当てです。");
        }
    }
    println!("\tGateway");
    let gateways = &adapter.gateway;
    if gateways.len() != 0 {
        for gateway in gateways {
            println!("\t\t{}", gateway);
        }
    } else {
        println!("\t\tGateway が未割り当てです。");
    }
    println!("\tDNS Server");
    let dns_servers = &adapter.dns_server;
    if dns_servers.len() != 0 {
        for dns_server in dns_servers {
            println!("\t\t{}", dns_server);
        }
    } else {
        println!("\t\tDNS Server が未割り当てです。");
    }
    {
        println!(
            "\t送信リンク速度: {}Bits / sec",
            to_human_readable(adapter.tx_link_speed_bps)
        );
        println!(
            "\t受信リンク速度: {}Bits / sec",
            to_human_readable(adapter.rx_link_speed_bps)
        );
    }
}
