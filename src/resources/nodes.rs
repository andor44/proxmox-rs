use serde_json;
use reqwest;
use super::{IntOrString, Parent, Subresource};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub cpu: f32,
    pub disk: u64,
    pub id: String,
    pub level: String,
    pub maxcpu: usize,
    pub maxdisk: u64,
    pub maxmem: u64,
    pub mem: u64,
    pub node: String,
    pub ssl_fingerprint: String,
    pub status: String,
    // not sure what's the point of this, maybe same serializer used as for /cluster/resources?
    #[serde(rename = "type")]
    pub type_: String,
    pub uptime: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Qemu {
    pub cpu: f32,
    pub cpus: usize,
    pub disk: u64,
    pub diskread: u64,
    pub diskwrite: u64,
    pub maxdisk: u64,
    pub maxmem: u64,
    pub mem: u64,
    pub name: String,
    pub netin: u64,
    pub netout: u64,
    pub pid: String,
    pub status: String,
    pub template: IntOrString,
    pub uptime: u64,
    pub vmid: usize,
}
