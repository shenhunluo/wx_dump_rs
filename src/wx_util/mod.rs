use std::{collections::HashMap, fs::File, io::Read};

use anyhow::{anyhow, Ok};
use byteorder::{LittleEndian, ReadBytesExt};
use windows::Win32::{
    Foundation::HANDLE,
    System::Diagnostics::ToolHelp::{MODULEENTRY32, PROCESSENTRY32},
};

use crate::util::{
    get_data_by_real_addr, get_module_by_name, get_process_by_id, get_process_by_name,
    get_process_handle, get_version,
};

fn get_wechat_info(wechat_info: &mut WeChatInfo) -> anyhow::Result<()> {
    wechat_info.version = get_version(&wechat_info.module)?;
    Ok(())
}

fn get_wechat_data(wechat_info: &mut WeChatInfo, offset: usize) -> anyhow::Result<String> {
    let buffer = get_data_by_real_addr(
        wechat_info.process.th32ProcessID,
        wechat_info.module.modBaseAddr as usize + offset,
        100,
    )?;
    return Ok(format!(
        "{}",
        String::from_utf8(buffer.split(|n| *n == 0).next().unwrap().to_vec())?
    ));
}

fn get_wechat_key(wechat_info: &mut WeChatInfo, offset: usize) -> anyhow::Result<[u8; 32]> {
    let buffer = get_data_by_real_addr(
        wechat_info.process.th32ProcessID,
        wechat_info.module.modBaseAddr as usize + offset,
        8,
    )?;
    let mut cur = std::io::Cursor::new(&buffer);
    let offset = cur.read_u64::<LittleEndian>()?;
    let key_buffer = get_data_by_real_addr(wechat_info.process.th32ProcessID, offset as usize, 32)?;
    return Ok(*&key_buffer[..].try_into()?);
}

pub fn open_wechat_process(
    wechat_info: &mut WeChatInfo,
    offset_map: &String,
    process_id: &Option<u32>,
    process_name: &String,
    module_name: &String,
) -> anyhow::Result<()> {
    wechat_info.process = if let Some(id) = process_id {
        get_process_by_id(*id)?
    } else {
        get_process_by_name(&process_name)?
    };
    wechat_info.handle = get_process_handle(wechat_info.process.th32ProcessID)?;

    wechat_info.module = get_module_by_name(wechat_info.process, &module_name)?;
    get_wechat_info(wechat_info)?;
    let mut offset_map_file = File::open(&offset_map)?;
    let mut buf = String::new();
    offset_map_file.read_to_string(&mut buf)?;
    let offset_map_map: HashMap<String, Vec<usize>> = serde_json::de::from_str(&buf)?;
    let offsets = offset_map_map
        .get(&wechat_info.version)
        .ok_or(anyhow!(format!(
            "微信版本为：{}，{}文件中并未找到该版本的偏移量",
            wechat_info.version, offset_map
        )))?;
    wechat_info.nick_name = get_wechat_data(wechat_info, offsets[0]).unwrap_or_else(|e| e.to_string());
    wechat_info.account = get_wechat_data(wechat_info, offsets[1]).unwrap_or_else(|e| e.to_string());
    wechat_info.mobile = get_wechat_data(wechat_info, offsets[2]).unwrap_or_else(|e| e.to_string());
    wechat_info.key = get_wechat_key(wechat_info, offsets[4])?;
    Ok(())
}

pub fn open_wechat_process_with_out_info(
    wechat_info: &mut WeChatInfo,
    process_id: &Option<u32>,
    process_name: &String,
    module_name: &String,
) -> anyhow::Result<()> {
    wechat_info.process = if let Some(id) = process_id {
        get_process_by_id(*id)?
    } else {
        get_process_by_name(&process_name)?
    };
    wechat_info.handle = get_process_handle(wechat_info.process.th32ProcessID)?;
    wechat_info.module = get_module_by_name(wechat_info.process, &module_name)?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct WeChatInfo {
    pub version: String,
    pub nick_name: String,
    pub account: String,
    pub mobile: String,
    pub key: [u8; 32],
    pub process: PROCESSENTRY32,
    pub handle: HANDLE,
    pub module: MODULEENTRY32,
}
