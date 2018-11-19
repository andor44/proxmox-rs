use reqwest;
use super::{IntOrString, Client};

#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub id: String,
    #[serde(rename = "msg")]
    pub message: String,
    pub node: String,
    pub pid: usize,
    #[serde(rename = "pri")]
    pub priority: usize,
    pub tag: String,
    pub time: usize,
    pub uid: String,
    pub user: String,
}

#[derive(Serialize, Default)]
pub struct LogOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NextId(String);

#[derive(Serialize, Default)]
pub struct NextIdOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmid: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MigrationOptions {
    pub network: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    pub keyboard: String,
    pub mac_prefix: String,
    pub migration: MigrationOptions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Resource {
    Pool {
        #[serde(rename = "pool")]
        name: String,
        uptime: usize,
        mem: u64,
        maxmem: u64,
        maxcpu: usize,
        id: String,
        cpu: f32,
    },
    Qemu {
        cpu: f32,
        disk: u64,
        diskread: u64,
        diskwrite: u64,
        id: String,
        maxcpu: usize,
        maxdisk: u64,
        maxmem: u64,
        mem: u64,
        name: String,
        netin: u64,
        netout: u64,
        node: String,
        pool: Option<String>,
        status: String,
        template: usize,
        uptime: usize,
        vmid: usize,
    },
    Node {
        cpu: f32,
        disk: u64,
        id: String,
        level: String,
        maxcpu: usize,
        maxdisk: u64,
        maxmem: u64,
        mem: u64,
        node: String,
        status: String,
        uptime: usize,
    },
    Storage {
        disk: u64,
        id: String,
        maxdisk: u64,
        node: String,
        status: String,
        storage: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    Vm,
    Storage,
    Node,
}

#[derive(Serialize, Default)]
pub struct ResourceOptions {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<ResourceType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Status {
    Cluster {
        id: String,
        name: String,
        nodes: usize,
        quorate: usize,
        version: usize,
    },
    Node {
        id: String,
        ip: String,
        level: String,
        local: usize,
        name: String,
        nodeid: usize,
        online: usize,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub endtime: usize,
    pub id: String,
    pub node: String,
    pub saved: String,
    // TODO: this was fixed in a later PVE version and is no longer inconsistent
    pub starttime: IntOrString,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub upid: String,
    pub user: String,
}
