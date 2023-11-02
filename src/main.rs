use action::{
    decrypt::{decrypt, dev},
    get_database::get_database,
    memory_search::{memory_search, memory_search_from_wechat_all_modules},
};
use anyhow::Ok;

mod action;
mod util;
mod wx_util;

use clap::{Parser, Subcommand};
use util::string_to_u8_vec;

use crate::{
    action::memory_search::{get_memory, memory_search_from_wechat_all_data},
    util::u8_to_string,
};

/// SharpWxDump、GoWxDump的rust语言版。包括获取信息，复制数据库，解密数据库，搜索内存等功能
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 指定偏移量文件
    #[arg(
        short,
        long,
        value_name = "json文件",
        default_value = "offset_map.json"
    )]
    offset_map: String,
    /// 指定微信聊天记录的文件夹，不填写时会默认指定系统文档文件夹下的WeChat Files文件夹
    #[arg(short = 'd', long, value_name = "文件夹")]
    wechat_dir: Option<String>,
    /// 请保证该文件夹内没有重要数据！复制微信记录前会清空该文件夹。指定保存路径，将会将微信记录复制到此路径下用于解密
    #[arg(
        long,
        value_name = "可能会被清空的文件夹",
        default_value = "wechat_database_for_decrypt"
    )]
    save_path: String,
    /// 解密后的数据将存放于该文件夹
    #[arg(
        long,
        value_name = "文件夹",
        default_value = "decrypted_wechat_database"
    )]
    decrypt_path: String,
    #[command(subcommand)]
    sub_commands: Option<SubCommands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    /// 显示当先登录的微信用户的信息
    ShowUserInfo,
    /// 复制聊天记录数据库
    GetDatabase {
        /// 登录名，将依次值寻找复制聊天记录数据库
        #[arg(short, long)]
        account: Option<String>,
    },
    /// 解密聊天记录数据库
    Decrypt {
        /// 获取到的key
        #[arg(short, long)]
        /// key的编码格式，可选值：[hex,base64,string]
        key: Option<String>,
        #[arg(short = 'd', long, default_value = "hex")]
        /// 是否在解密时检查hmac
        encode: String,
        #[arg(long, default_value = "false")]
        check_hmac: bool,
    },
    /// 从内存中搜索指定信息
    Search {
        /// 要搜索的内容
        #[arg(short, long)]
        str: String,
        /// key的编码格式，可选值：[hex,base64,string]
        #[arg(short = 'd', long, default_value = "string")]
        encode: String,
        /// 返回真实地址
        #[arg(long)]
        real_addr: bool,
        /// 从微信的所有模块中搜索数据
        #[arg(long)]
        from_all_modules: bool,
        /// 从微信的所有数据中搜索数据
        #[arg(long)]
        from_all_data: bool,
        /// 显示未搜索到的数据信息
        #[arg(long)]
        show_no_found_info: bool,
    },
    /// 从内存中指定的位置搜索信息
    GetMemory {
        /// 想要从哪个位置获取值
        #[arg(short, long)]
        index: usize,
        /// 想要获取的值的长度
        #[arg(short, long)]
        len: usize,
        /// 返回值的的编码格式，可选值：[hex, base64, string, u64be, u64le, u32be, u32le, u16be, u16le, i64be, i64le, i32be, i32le, i164be, i16le]
        #[arg(short = 'd', long)]
        encode: Option<String>,
        /// index为真实地址
        #[arg(long)]
        real_addr: bool,
    },
    /// 开发用，无功能
    Dev,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.sub_commands {
        Some(sub_commands) => match sub_commands {
            SubCommands::ShowUserInfo => {
                let mut wechat_info = wx_util::WeChatInfo::default();
                wx_util::open_wechat_process(&mut wechat_info, &args.offset_map)?;
                action::show_info::show_user_info(&wechat_info)?;
            }
            SubCommands::GetDatabase { account } => {
                let mut wechat_info = wx_util::WeChatInfo::default();
                if let Some(account) = account {
                    wechat_info.account = account;
                } else {
                    wx_util::open_wechat_process(&mut wechat_info, &args.offset_map)?;
                }
                get_database(&args.wechat_dir, &args.save_path, &wechat_info.account)?;
            }
            SubCommands::Decrypt {
                key,
                encode,
                check_hmac,
            } => {
                let mut wechat_info = wx_util::WeChatInfo::default();
                if let Some(key) = key {
                    let key_vec = string_to_u8_vec(&key, &encode)?;
                    if key_vec.len() < 32 {
                        return Err(anyhow::anyhow!("请输入正确的key"));
                    } else {
                        wechat_info.key = key_vec[0..32].try_into()?;
                    }
                } else {
                    wx_util::open_wechat_process(&mut wechat_info, &args.offset_map)?;
                };
                decrypt(
                    &args.save_path,
                    &args.decrypt_path,
                    &wechat_info.key,
                    check_hmac,
                )?;
            }
            SubCommands::Search {
                str,
                encode,
                real_addr,
                from_all_data,
                from_all_modules,
                show_no_found_info,
            } => {
                let mut wechat_info = wx_util::WeChatInfo::default();
                wx_util::open_wechat_process(&mut wechat_info, &args.offset_map)?;
                let data = string_to_u8_vec(&str, &encode)?;
                if from_all_data {
                    memory_search_from_wechat_all_data(&wechat_info, &data, real_addr,show_no_found_info)?;
                }
                if from_all_modules {
                    memory_search_from_wechat_all_modules(&wechat_info, &data, real_addr,show_no_found_info)?;
                }
                if !(from_all_modules || from_all_data) {
                    println!("{:?}", memory_search(&wechat_info, &data, real_addr)?);
                }
            }
            SubCommands::GetMemory {
                index,
                len,
                encode,
                real_addr,
            } => {
                let mut wechat_info = wx_util::WeChatInfo::default();
                wx_util::open_wechat_process(&mut wechat_info, &args.offset_map)?;
                let data = get_memory(&wechat_info, index, len, real_addr)?;
                if let Some(encode) = encode {
                    println!("{}", u8_to_string(&data, &encode)?);
                } else {
                    println!("{:?}", &data);
                };
            }
            SubCommands::Dev => {
                dev()?;
            }
        },
        None => {
            let mut wechat_info = wx_util::WeChatInfo::default();
            wx_util::open_wechat_process(&mut wechat_info, &args.offset_map)?;
            action::show_info::show_user_info(&wechat_info)?;
            get_database(&args.wechat_dir, &args.save_path, &wechat_info.account)?;
            decrypt(&args.save_path, &args.decrypt_path, &wechat_info.key, false)?;
        }
    }
    Ok(())
}
