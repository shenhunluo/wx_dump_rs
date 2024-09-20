use std::sync::Arc;

use iced::{
    widget::{Button, Column, Container, Row, Space},
    Length,
};

use crate::{
    action::decrypt::decrypt,
    util::{self, string_to_u8_vec, u8_to_string},
};

use super::{
    config_body::ConfigBody,
    gui_util::{set_col_with_text, set_col_with_text_input, PrintInfoText},
    show_user_info_body::ShowUserInfoBody,
    Message,
};

pub struct DecryptBody {
    save_path: String,
    decrypt_path: String,
    key: Option<[u8; 32]>,
    input_base64_key: String,
    input_base64_key_err_msg: Option<String>,
    input_hex_key: String,
    input_hex_key_err_msg: Option<String>,
    command_running: bool,
    print_info_text: Arc<PrintInfoText>,
}

impl DecryptBody {
    pub fn new() -> Self {
        Self {
            save_path: String::new(),
            decrypt_path: String::new(),
            key: None,
            input_base64_key: String::new(),
            input_hex_key: String::new(),
            command_running: false,
            print_info_text: Arc::new(PrintInfoText::new(iced::widget::scrollable::Id::new(
                "decrypt_body_print_info",
            ))),
            input_base64_key_err_msg: None,
            input_hex_key_err_msg: None,
        }
    }
    pub fn check_command_running(&self) -> bool {
        self.command_running
    }
    pub fn check_scroll(&self) -> iced::Task<Message> {
        self.print_info_text.check_scroll()
    }
    pub fn update(
        &mut self,
        msg: DecryptMessage,
        config: &ConfigBody,
        show_user_info: &ShowUserInfoBody,
    ) -> iced::Task<Message> {
        match msg {
            DecryptMessage::UpdateDecrypt => {
                self.save_path = config.save_path.trim().to_owned();
                self.decrypt_path = config.decrypt_path.trim().to_owned();
                if let Some(key) = show_user_info.get_key() {
                    self.key = Some(key.clone());
                    if let Ok(s) = util::u8_to_string(&key[..], &"hex".to_owned()) {
                        self.input_hex_key = s;
                        self.input_hex_key_err_msg = None;
                    }
                    if let Ok(s) = util::u8_to_string(&key[..], &"base64".to_owned()) {
                        self.input_base64_key = s;
                        self.input_base64_key_err_msg = None;
                    }
                }
                iced::Task::none()
            }
            DecryptMessage::ButtonStart => match &self.key {
                Some(key) => {
                    let key = key.clone();
                    self.command_running = true;
                    let print_info_text = self.print_info_text.clone();
                    let save_path = self.save_path.clone();
                    let decrypt_path = self.decrypt_path.clone();
                    iced::Task::<Message>::perform(
                        async move {
                            match decrypt(&save_path, &decrypt_path, &key, false, |s| {
                                print_info_text.push_new_len(s);
                            }) {
                                Ok(_) => {
                                    Message::DecryptMessage(DecryptMessage::CommandFinished(Ok(())))
                                }
                                Err(e) => Message::DecryptMessage(DecryptMessage::CommandFinished(
                                    Err(e.to_string()),
                                )),
                            }
                        },
                        |msg| msg,
                    )
                }
                None => {
                    self.print_info_text.push_new_err_len("请输入正确的key");
                    iced::Task::none()
                }
            },
            DecryptMessage::InputBase64Key(s) => {
                self.input_base64_key = s;
                self.input_base64_key_err_msg = None;
                iced::Task::none()
            }
            DecryptMessage::ButtonBase64Key => {
                match string_to_u8_vec(
                    &self.input_base64_key.trim().to_owned(),
                    &"base64".to_string(),
                ) {
                    Ok(s) => {
                        if s.len() != 32 {
                            self.input_base64_key_err_msg = Some("key的长度错误".to_owned());
                        } else {
                            self.input_base64_key_err_msg = None;
                            self.input_hex_key_err_msg = None;
                            self.input_base64_key = u8_to_string(&s, &"base64".to_owned()).unwrap();
                            self.input_hex_key = u8_to_string(&s, &"hex".to_owned()).unwrap();
                            self.key = Some(s.try_into().unwrap())
                        }
                    }
                    Err(e) => {
                        self.input_base64_key_err_msg =
                            Some(format!("key格式化失败,错误信息:{}", e.to_string()))
                    }
                }
                iced::Task::none()
            }
            DecryptMessage::InputHexKey(s) => {
                self.input_hex_key = s;
                self.input_hex_key_err_msg = None;
                iced::Task::none()
            }
            DecryptMessage::ButtonHexKey => {
                match string_to_u8_vec(&self.input_hex_key.trim().to_owned(), &"hex".to_string()) {
                    Ok(s) => {
                        if s.len() != 32 {
                            self.input_hex_key_err_msg = Some("key的长度错误".to_owned());
                        } else {
                            self.input_hex_key_err_msg = None;
                            self.input_base64_key_err_msg = None;
                            self.input_base64_key = u8_to_string(&s, &"base64".to_owned()).unwrap();
                            self.input_hex_key = u8_to_string(&s, &"hex".to_owned()).unwrap();
                            self.key = Some(s.try_into().unwrap())
                        }
                    }
                    Err(e) => {
                        self.input_hex_key_err_msg =
                            Some(format!("key格式化失败,错误信息:{}", e.to_string()))
                    }
                }
                iced::Task::none()
            }
            DecryptMessage::CommandFinished(r) => {
                self.command_running = false;
                if let Err(e) = r {
                    self.print_info_text.push_new_err_len(e);
                }
                self.print_info_text.check_scroll()
            }
        }
    }

    pub fn draw(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(5);
            col = set_col_with_text(
                col,
                "备份文件夹",
                None::<Message>,
                &self.save_path,
                "",
                &None::<String>,
            );
            col = set_col_with_text(
                col,
                "解密文件夹",
                None::<Message>,
                &self.decrypt_path,
                "",
                &None::<String>,
            );
            col = set_col_with_text(
                col,
                "key",
                None::<Message>,
                &self
                    .key
                    .map(|s| {
                        s.iter()
                            .map(|u| u.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    })
                    .unwrap_or("".to_owned()),
                "",
                &None::<String>,
            );
            col = set_col_with_text_input(
                col,
                "key(base64)",
                |s| DecryptMessage::InputBase64Key(s),
                Some(DecryptMessage::ButtonBase64Key),
                &self.input_base64_key,
                "设置",
                &self.input_base64_key_err_msg,
            );
            col = set_col_with_text_input(
                col,
                "key(hex)",
                |s| DecryptMessage::InputHexKey(s),
                Some(DecryptMessage::ButtonHexKey),
                &self.input_hex_key,
                "设置",
                &self.input_hex_key_err_msg,
            );
            col = col.push(self.print_info_text.draw().height(Length::Fill));
            col = col.push(Row::new().spacing(5).push(Space::with_width(10)).push(
                if self.command_running {
                    Button::new("解密运行中")
                } else {
                    Button::new("开始解密")
                        .on_press(Message::DecryptMessage(DecryptMessage::ButtonStart))
                },
            ));
            col = col.push(Space::with_height(5));
            col
        })
    }
}

#[derive(Debug, Clone)]
pub enum DecryptMessage {
    UpdateDecrypt,
    ButtonStart,
    InputBase64Key(String),
    ButtonBase64Key,
    InputHexKey(String),
    ButtonHexKey,
    CommandFinished(Result<(), String>),
}

impl Into<Message> for DecryptMessage {
    fn into(self) -> Message {
        Message::DecryptMessage(self)
    }
}
