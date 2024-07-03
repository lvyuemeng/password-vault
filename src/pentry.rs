use std::fs::OpenOptions;
use std::io::Read;
use std::vec;

use crate::call_control_flow;
use crate::errors::*;
use crate::untils::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub service: String,
    username: String,
    password: String,
}

type VecInfo = Vec<Info>;

impl Info {
    pub fn new(sv: String, un: String, pw: String) -> Info {
        Info {
            service: sv,
            username: un,
            password: pw,
        }
    }

    pub fn info_read() -> Result<Control<Info>> {
        let sv = call_control_flow!(read_input("Please input your service: ")?);
        let un = call_control_flow!(read_input("Please input your username: ")?);
        let pw = call_control_flow!(read_input("Please input your password: ")?);

        let info = Self::new(sv, un, pw);
        Ok(Control::Next(info))
    }
}

pub fn load_from_file(file_path: &str) -> Result<VecInfo> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    let mut infos = String::new();
    file.read_to_string(&mut infos)?;
    let vaults: VecInfo = serde_json::from_str(&infos).unwrap_or_else(|_| vec![]);
    Ok(vaults)
}

pub fn save_to_file(vaults: &VecInfo, file_path: &str) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    serde_json::to_writer(file, vaults)?;
    Ok(())
}
