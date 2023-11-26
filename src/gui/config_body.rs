use iced::widget::{Container, Column};

use crate::util::{get_file_dialog, get_folder_dialog};

use super::{Message, gui_util::set_col_with_text_input};

#[derive(Debug, Clone)]
pub struct ConfigBody {
    pub offset_map: String,
    pub offset_msg: Option<String>,
    pub wechat_dir: Option<String>,
    pub wechat_dir_msg: Option<String>,
    pub save_path: String,
    pub save_path_msg: Option<String>,
    pub decrypt_path: String,
    pub decrypt_path_msg: Option<String>,
    pub process_id: Option<u32>,
    pub process_name: String,
    pub module_name: String,
}

impl ConfigBody {
    pub fn new() -> Self {
        ConfigBody {
            offset_map: "offset_map.json".to_owned(),
            offset_msg: None,
            wechat_dir: None,
            wechat_dir_msg: None,
            save_path: "wechat_database_for_decrypt".to_owned(),
            decrypt_path: "decrypted_wechat_database".to_owned(),
            process_id: None,
            process_name: "WeChat.exe".to_owned(),
            module_name: "WeChatWin.dll".to_owned(),
            save_path_msg: None,
            decrypt_path_msg: None,
        }
    }
    pub fn update(&mut self, msg: ConfigMessage) {
        match msg {
            ConfigMessage::InputOffsetMap(s) => {
                self.offset_map = s;
                self.offset_msg = None;
            },
            ConfigMessage::ButtonOffsetMap => {
                match get_file_dialog() {
                    Ok(s) => {
                        self.offset_map = s;
                        self.offset_msg = None;
                    },
                    Err(e) => {
                        self.offset_msg = Some(e.to_string());
                    },
                }
            },
            ConfigMessage::InputWeChatDir(s) => {
                let s = s;
                if s.len() == 0 {
                    self.wechat_dir = None;
                } else {
                    self.wechat_dir = Some(s);
                }
                self.wechat_dir_msg = None;
            },
            ConfigMessage::ButtonWeChatDir => {
                match get_folder_dialog() {
                    Ok(folder) => {
                        self.wechat_dir = Some(folder);
                        self.wechat_dir_msg = None;
                    },
                    Err(err) => {
                        self.wechat_dir_msg = Some(err.to_string());
                    }
                }
            },
            ConfigMessage::InputSavePath(s) => {
                self.save_path = s;
                self.save_path_msg = None;
            },
            ConfigMessage::ButtonSavePath => {
                match get_folder_dialog() {
                    Ok(folder) => {
                        self.save_path = folder;
                        self.save_path_msg = None;
                    },
                    Err(err) => {
                        self.save_path_msg = Some(err.to_string());
                    }
                }
            },
            ConfigMessage::InputDecryptPath(s) => {
                self.decrypt_path = s;
                self.decrypt_path_msg = None;
            },
            ConfigMessage::ButtonDecryptPath => {
                match get_folder_dialog() {
                    Ok(folder) => {
                        self.decrypt_path = folder;
                        self.decrypt_path_msg = None;
                    },
                    Err(err) => {
                        self.decrypt_path_msg = Some(err.to_string());
                    }
                }
            },
            ConfigMessage::InputProcessId(s) => {
                if s.trim().len() == 0 {
                    self.process_id = None;
                } else {
                    if let Some(s) = s.parse().ok() {
                        self.process_id = Some(s);
                    }
                }
            },
            ConfigMessage::InputProcessName(s) => {
                self.process_name = s;
            },
            ConfigMessage::InputModuleName(s) => {
                self.module_name = s;
            },
        }
    } 
    pub fn draw(&self) -> Container<Message> {
        Container::new(
            {
                let mut col = Column::new()
                .spacing(5);
                col = set_col_with_text_input(col, "偏移量文件", |s| ConfigMessage::InputOffsetMap(s), Some(ConfigMessage::ButtonOffsetMap), &self.offset_map, "...", &self.offset_msg);
                col = set_col_with_text_input(col, "微信文件夹（空值为默认文件夹）", |s| ConfigMessage::InputWeChatDir(s), Some(ConfigMessage::ButtonWeChatDir), &self.wechat_dir.as_ref().unwrap_or(&"".to_string()), "...", &self.wechat_dir_msg);
                col = set_col_with_text_input(col, "数据库备份文件夹", |s| ConfigMessage::InputSavePath(s), Some(ConfigMessage::ButtonSavePath), &&self.save_path, "...", &self.save_path_msg);
                col = set_col_with_text_input(col, "数据库解密文件夹", |s| ConfigMessage::InputDecryptPath(s), Some(ConfigMessage::ButtonDecryptPath), &self.decrypt_path, "...", &self.decrypt_path_msg);
                col = set_col_with_text_input(col, "微信进程号（可为空）", |s| ConfigMessage::InputProcessId(s), None, &self.process_id.as_ref().map(|p| p.to_string()).unwrap_or("".to_string()), "...", &None::<String>);
                col = set_col_with_text_input(col, "微信进程名", |s| ConfigMessage::InputProcessName(s), None, &self.process_name, "...", &None::<String>);
                col = set_col_with_text_input(col, "微信模块名", |s| ConfigMessage::InputModuleName(s), None, &self.module_name, "...", &None::<String>);
                col
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum ConfigMessage {
    InputOffsetMap(String),
    ButtonOffsetMap,
    InputWeChatDir(String),
    ButtonWeChatDir,
    InputSavePath(String),
    ButtonSavePath,
    InputDecryptPath(String),
    ButtonDecryptPath,
    InputProcessId(String),
    InputProcessName(String),
    InputModuleName(String),
}

impl Into<Message> for ConfigMessage {
    fn into(self) -> Message {
        Message::ConfigMessage(self)
    }
}