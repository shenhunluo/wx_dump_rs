use std::{collections::HashMap, ffi::c_void, fs::File, io::Read};

use anyhow::{anyhow, Ok};
use byteorder::{LittleEndian, ReadBytesExt};
use windows::core::PCSTR;
use windows::Win32::{
    Foundation::{GetLastError, HANDLE},
    Storage::FileSystem::{VerQueryValueA, VS_FIXEDFILEINFO},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Module32Next, Process32Next, Toolhelp32ReadProcessMemory,
            MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
        },
        LibraryLoader::{FindResourceA, LoadLibraryExA, LoadResource, LOAD_LIBRARY_AS_DATAFILE},
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
    UI::WindowsAndMessaging::RT_VERSION,
};

fn get_wechat_process() -> anyhow::Result<PROCESSENTRY32> {
    unsafe {
        let mut process = PROCESSENTRY32::default();
        process.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        let process_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        loop {
            Process32Next(process_snapshot, &mut process)?;
            if process.szExeFile.split(|n| *n == 0).next().unwrap() == "WeChat.exe".as_bytes() {
                return Ok(process);
            }
        }
    }
}

fn get_wechat_module(process: PROCESSENTRY32) -> anyhow::Result<MODULEENTRY32> {
    let mut module = MODULEENTRY32::default();
    module.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process.th32ProcessID)?;
        loop {
            Module32Next(snapshot, &mut module)?;
            if module.szModule.split(|n| *n == 0).next().unwrap() == "WeChatWin.dll".as_bytes() {
                return Ok(module);
            }
        }
    }
}

pub fn get_wechat_modules(process: PROCESSENTRY32) -> anyhow::Result<Vec<MODULEENTRY32>> {
    let mut vec = vec![];
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process.th32ProcessID)?;
        loop {
            let mut module = MODULEENTRY32::default();
            module.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
            let r = Module32Next(snapshot, &mut module);
            if r.is_ok() {
                vec.push(module);
            } else {
                break;
            }
        }
    }
    Ok(vec)
}

fn get_wechat_version(module: &MODULEENTRY32) -> anyhow::Result<String> {
    unsafe {
        let image = LoadLibraryExA(
            PCSTR::from_raw(module.szExePath.as_ptr()),
            HANDLE::default(),
            LOAD_LIBRARY_AS_DATAFILE,
        )?;
        let res_info = FindResourceA(image, PCSTR(1u8 as _), PCSTR(RT_VERSION.as_ptr() as _))?;
        let res_data = LoadResource(image, res_info)?;
        let mut info = VS_FIXEDFILEINFO::default();
        let mut info_ref = &mut info as *mut _ as *mut c_void;
        let mut size = std::mem::size_of::<VS_FIXEDFILEINFO>() as u32;
        let b = VerQueryValueA(
            res_data.0,
            PCSTR::from_raw("\0".as_bytes().as_ptr()),
            &mut info_ref as *mut *mut c_void,
            &mut size,
        );
        let info_ref = info_ref as *mut VS_FIXEDFILEINFO;
        info = *info_ref;
        if b.0 == 0 {
            GetLastError()?;
        }
        return Ok(format!(
            "{}.{}.{}.{}",
            info.dwFileVersionMS >> 16,
            info.dwFileVersionMS & 0xffff,
            info.dwFileVersionLS >> 16,
            info.dwFileVersionLS & 0xffff
        ));
    }
}

fn get_wechat_info(wechat_info: &mut WeChatInfo) -> anyhow::Result<()> {
    wechat_info.version = get_wechat_version(&wechat_info.module)?;
    Ok(())
}

fn get_wechat_data(wechat_info: &mut WeChatInfo, offset: usize) -> anyhow::Result<String> {
    unsafe {
        let mut buffer = [0u8; 100];
        let mut numberofbytesread = 0;
        let base_addr = wechat_info.module.modBaseAddr as usize;
        let b = Toolhelp32ReadProcessMemory(
            wechat_info.process.th32ProcessID,
            (base_addr + offset) as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            100,
            &mut numberofbytesread,
        );
        if b.0 == 0 {
            GetLastError()?;
        }
        return Ok(format!(
            "{}",
            String::from_utf8(buffer.split(|n| *n == 0).next().unwrap().to_vec())?
        ));
    }
}

pub fn get_wechat_data_by_real_addr(
    wechat_info: &WeChatInfo,
    offset: usize,
    mut len: usize,
) -> anyhow::Result<Vec<u8>> {
    let mut vec = vec![];
    unsafe {
        for i in 0..wechat_info.module.modBaseSize as usize / 4096 {
            let mut buffer = [0u8; 4096];
            let mut numberofbytesread = 0;
            if len > 4096 {
                let b = Toolhelp32ReadProcessMemory(
                    wechat_info.process.th32ProcessID,
                    (offset + i * 4096) as *const c_void,
                    buffer.as_mut_ptr() as *mut c_void,
                    4096,
                    &mut numberofbytesread,
                );
                if b.0 == 0 {
                    GetLastError()?;
                }
                vec.append(&mut buffer.to_vec());
                len = len - 4096;
            } else {
                let b = Toolhelp32ReadProcessMemory(
                    wechat_info.process.th32ProcessID,
                    (offset + i * 4096) as *const c_void,
                    buffer.as_mut_ptr() as *mut c_void,
                    len,
                    &mut numberofbytesread,
                );
                if b.0 == 0 {
                    GetLastError()?;
                }
                vec.append(&mut buffer[0..len].to_vec());
                break;
            }
        }
    }
    Ok(vec)
}

fn get_wechat_key(wechat_info: &mut WeChatInfo, offset: usize) -> anyhow::Result<[u8; 32]> {
    unsafe {
        let mut buffer = [0u8; 8];
        let mut numberofbytesread = 0;
        let base_addr = wechat_info.module.modBaseAddr as usize;
        let b = Toolhelp32ReadProcessMemory(
            wechat_info.process.th32ProcessID,
            (base_addr + offset) as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            8,
            &mut numberofbytesread,
        );
        if b.0 == 0 {
            GetLastError()?;
        }
        let mut key_buffer = [0u8; 32];
        let mut cur = std::io::Cursor::new(&buffer);
        let offset = cur.read_u64::<LittleEndian>()?;
        let b = Toolhelp32ReadProcessMemory(
            wechat_info.process.th32ProcessID,
            offset as *const c_void,
            key_buffer.as_mut_ptr() as *mut c_void,
            32,
            &mut numberofbytesread,
        );
        if b.0 == 0 {
            GetLastError()?;
        };
        return Ok(key_buffer);
    }
}

pub fn open_wechat_process(
    wechat_info: &mut WeChatInfo,
    offset_map: &String,
) -> anyhow::Result<()> {
    wechat_info.process = get_wechat_process()?;
    wechat_info.handle =
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, wechat_info.process.th32ProcessID) }?;
    wechat_info.module = get_wechat_module(wechat_info.process)?;
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
    wechat_info.nick_name = get_wechat_data(wechat_info, offsets[0])?;
    wechat_info.account = get_wechat_data(wechat_info, offsets[1])?;
    wechat_info.mobile = get_wechat_data(wechat_info, offsets[2])?;
    wechat_info.key = get_wechat_key(wechat_info, offsets[4])?;
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
