use std::ffi::c_void;

use anyhow::anyhow;
use base64::Engine;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{GetLastError, HANDLE},
        Storage::FileSystem::{VerQueryValueA, VS_FIXEDFILEINFO},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Module32Next, Process32Next, Toolhelp32ReadProcessMemory,
                MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
            },
            LibraryLoader::{
                FindResourceA, LoadLibraryExA, LoadResource, LOAD_LIBRARY_AS_DATAFILE,
            },
            Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION},
            Threading::{OpenProcess, PROCESS_ALL_ACCESS},
        },
        UI::WindowsAndMessaging::RT_VERSION,
    },
};
pub fn string_to_u8_vec(data: &String, encode: &String) -> anyhow::Result<Vec<u8>> {
    let mut buffer = vec![];
    match encode.to_ascii_lowercase().as_str() {
        "hex" => {
            for i in (0..data.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&data[i..i + 2], 16))
            {
                let i = i?;
                buffer.push(i);
            }
            Ok(buffer)
        }
        "base64" => {
            buffer = base64::engine::general_purpose::STANDARD_NO_PAD.decode(&data)?;
            Ok(buffer)
        }
        "string" => {
            buffer = data.as_bytes().to_vec();
            Ok(buffer)
        }
        "u64be" => {
            let i = data.parse::<u64>()?;
            Ok(i.to_be_bytes().to_vec())
        }
        "u64le" => {
            let i = data.parse::<u64>()?;
            Ok(i.to_le_bytes().to_vec())
        }
        "u32be" => {
            let i = data.parse::<u32>()?;
            Ok(i.to_be_bytes().to_vec())
        }
        "u32le" => {
            let i = data.parse::<u32>()?;
            Ok(i.to_le_bytes().to_vec())
        }
        "u16be" => {
            let i = data.parse::<u16>()?;
            Ok(i.to_be_bytes().to_vec())
        }
        "u16le" => {
            let i = data.parse::<u16>()?;
            Ok(i.to_le_bytes().to_vec())
        }
        "i64be" => {
            let i = data.parse::<i64>()?;
            Ok(i.to_be_bytes().to_vec())
        }
        "i64le" => {
            let i = data.parse::<i64>()?;
            Ok(i.to_le_bytes().to_vec())
        }
        "i32be" => {
            let i = data.parse::<i32>()?;
            Ok(i.to_be_bytes().to_vec())
        }
        "i32le" => {
            let i = data.parse::<i32>()?;
            Ok(i.to_le_bytes().to_vec())
        }
        "i16be" => {
            let i = data.parse::<i16>()?;
            Ok(i.to_be_bytes().to_vec())
        }
        "i16le" => {
            let i = data.parse::<i16>()?;
            Ok(i.to_le_bytes().to_vec())
        }
        _ => Err(anyhow!("错误的编码格式")),
    }
}

pub fn u8_to_string(data: &[u8], encode: &String) -> anyhow::Result<String> {
    match encode.to_ascii_lowercase().as_str() {
        "hex" => Ok(to_hex(data)),
        "base64" => Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(&data)),
        "string" => Ok(String::from_utf8(
            data.split(|e| *e == 0).next().unwrap().to_vec(),
        )?),
        "u64be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u64::<BigEndian>()?.to_string())
        }
        "u64le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u64::<LittleEndian>()?.to_string())
        }
        "u32be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u32::<BigEndian>()?.to_string())
        }
        "u32le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u32::<LittleEndian>()?.to_string())
        }
        "u16be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u16::<BigEndian>()?.to_string())
        }
        "u16le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_u16::<LittleEndian>()?.to_string())
        }
        "i64be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i64::<BigEndian>()?.to_string())
        }
        "i64le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i64::<LittleEndian>()?.to_string())
        }
        "i32be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i32::<BigEndian>()?.to_string())
        }
        "i32le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i32::<LittleEndian>()?.to_string())
        }
        "i16be" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i16::<BigEndian>()?.to_string())
        }
        "i16le" => {
            let mut cur = std::io::Cursor::new(data);
            Ok(cur.read_i16::<LittleEndian>()?.to_string())
        }
        _ => Err(anyhow!("错误的编码格式")),
    }
}

pub fn to_hex(data: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    for i in data.as_ref() {
        result.push(HEX_TABLE[(i >> 4) as usize]);
        result.push(HEX_TABLE[(i & 0x0F) as usize]);
    }
    result
}

pub static HEX_TABLE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub fn get_process_by_name(process_name: &str) -> anyhow::Result<PROCESSENTRY32> {
    unsafe {
        let mut process = PROCESSENTRY32::default();
        process.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        let process_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        loop {
            Process32Next(process_snapshot, &mut process)?;
            if process
                .szExeFile
                .split(|n| *n == 0)
                .next()
                .unwrap()
                .iter()
                .map(|i| *i as u8)
                .collect::<Vec<_>>()
                == process_name.as_bytes()
            {
                return Ok(process);
            }
        }
    }
}

pub fn get_process_by_id(process_id: u32) -> anyhow::Result<PROCESSENTRY32> {
    unsafe {
        let mut process = PROCESSENTRY32::default();
        process.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        let process_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
        loop {
            Process32Next(process_snapshot, &mut process)?;
            if process.th32ProcessID == process_id {
                return Ok(process);
            }
        }
    }
}

pub fn get_module_by_name(
    process: &PROCESSENTRY32,
    module_name: &str,
) -> anyhow::Result<MODULEENTRY32> {
    let mut module = MODULEENTRY32::default();
    module.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, process.th32ProcessID)?;
        loop {
            Module32Next(snapshot, &mut module)?;
            if module
                .szModule
                .split(|n| *n == 0)
                .next()
                .unwrap()
                .iter()
                .map(|i| *i as u8)
                .collect::<Vec<_>>()
                == module_name.as_bytes()
            {
                return Ok(module);
            }
        }
    }
}

pub fn get_modules(process: &PROCESSENTRY32) -> anyhow::Result<Vec<MODULEENTRY32>> {
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

pub fn get_version(module: &MODULEENTRY32) -> anyhow::Result<String> {
    unsafe {
        let image = LoadLibraryExA(
            PCSTR::from_raw(module.szExePath.as_ptr() as *const u8),
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
            GetLastError().ok()?;
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

pub fn get_data_by_real_addr(
    process_id: u32,
    offset: usize,
    mut len: usize,
) -> anyhow::Result<Vec<u8>> {
    let mut vec = vec![];
    unsafe {
        for i in 0..=len / 4096 {
            let mut buffer = [0u8; 4096];
            let mut numberofbytesread = 0;
            if len > 4096 {
                let b = Toolhelp32ReadProcessMemory(
                    process_id,
                    (offset + i * 4096) as *const c_void,
                    buffer.as_mut_ptr() as *mut c_void,
                    4096,
                    &mut numberofbytesread,
                );
                if b.0 == 0 {
                    GetLastError().ok()?;
                }
                vec.append(&mut buffer.to_vec());
                len = len - 4096;
            } else {
                let b = Toolhelp32ReadProcessMemory(
                    process_id,
                    (offset + i * 4096) as *const c_void,
                    buffer.as_mut_ptr() as *mut c_void,
                    len,
                    &mut numberofbytesread,
                );
                if b.0 == 0 {
                    GetLastError().ok()?;
                }
                vec.append(&mut buffer[0..len].to_vec());
                break;
            }
        }
    }
    Ok(vec)
}

pub fn get_process_handle(process_id: u32) -> anyhow::Result<HANDLE> {
    Ok(unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, process_id) }?)
}

pub fn get_all_memory_by_handle(handle: &HANDLE) -> anyhow::Result<Vec<(usize, usize)>> {
    let mut lp_addr = None;
    let mut vec = vec![];

    loop {
        let mut memory_basic_info = MEMORY_BASIC_INFORMATION::default();
        unsafe {
            let r = VirtualQueryEx(
                *handle,
                lp_addr,
                &mut memory_basic_info,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );
            if r == 0 {
                break;
            }
        }
        let base_addr = memory_basic_info.BaseAddress as usize;
        let size = memory_basic_info.RegionSize as usize;
        lp_addr = Some((base_addr + size) as *const c_void);
        vec.push((base_addr, size))
    }

    Ok(vec)
}

#[cfg(feature = "gui")]
pub fn get_file_dialog() -> Result<String, anyhow::Error> {
    use windows::Win32::{
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::Shell::{IFileDialog, SIGDN_FILESYSPATH},
    };
    unsafe {
        let file_dialog: IFileDialog = CoCreateInstance(
            &windows::core::GUID::from_u128(0xdc1c5a9ce88a4ddea5a160f82a20aef7),
            None,
            CLSCTX_INPROC_SERVER,
        )?;
        file_dialog.Show(None)?;
        let result = file_dialog.GetResult()?;
        let result = result.GetDisplayName(SIGDN_FILESYSPATH)?.to_string()?;
        Ok(result)
    }
}

#[cfg(feature = "gui")]
pub fn get_folder_dialog() -> Result<String, anyhow::Error> {
    use windows::Win32::{
        System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER},
        UI::Shell::{IFileDialog, FOS_PICKFOLDERS, SIGDN_FILESYSPATH},
    };
    unsafe {
        let file_dialog: IFileDialog = CoCreateInstance(
            &windows::core::GUID::from_u128(0xdc1c5a9ce88a4ddea5a160f82a20aef7),
            None,
            CLSCTX_INPROC_SERVER,
        )?;
        file_dialog.SetOptions(FOS_PICKFOLDERS)?;
        file_dialog.Show(None)?;
        let result = file_dialog.GetResult()?;
        let result = result.GetDisplayName(SIGDN_FILESYSPATH)?.to_string()?;
        Ok(result)
    }
}
