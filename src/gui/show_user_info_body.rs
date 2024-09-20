use std::sync::Arc;

use iced::{
    widget::{Button, Column, Container, Row, Space},
    Length,
};

use crate::{action::show_info::show_user_info, util::u8_to_string, wx_util};

use super::{config_body::ConfigBody, gui_util::PrintInfoText, Message};

pub struct ShowUserInfoBody {
    print_info_text: Arc<PrintInfoText>,
    key: Option<[u8; 32]>,
    account: Option<String>,
    command_running: bool,
}

impl ShowUserInfoBody {
    pub fn new() -> Self {
        Self {
            print_info_text: Arc::new(PrintInfoText::new(iced::widget::scrollable::Id::new(
                "show_user_info_print_info",
            ))),
            key: None,
            account: None,
            command_running: false,
        }
    }
    pub fn get_key(&self) -> &Option<[u8; 32]> {
        &self.key
    }
    pub fn get_account(&self) -> &Option<String> {
        &self.account
    }
    pub fn check_command_running(&self) -> bool {
        self.command_running
    }
    pub fn check_scroll(&self) -> iced::Task<Message> {
        self.print_info_text.check_scroll()
    }
    pub fn update(
        &mut self,
        msg: ShowUserInfoMessage,
        config: &ConfigBody,
    ) -> iced::Task<Message> {
        match msg {
            ShowUserInfoMessage::UpdateWeChatInfo => {
                self.command_running = true;
                let print_info_text = self.print_info_text.clone();
                let config = config.clone();
                iced::Task::<Message>::perform(
                    async move {
                        let mut wechat_info = wx_util::WeChatInfo::default();
                        let wechat_info = wx_util::open_wechat_process(
                            &mut wechat_info,
                            &config.offset_map,
                            &config.process_id,
                            &config.process_name,
                            &config.module_name,
                        )
                        .map(|_| wechat_info);
                        match wechat_info {
                            Ok(wechat_info) => {
                                match show_user_info(&wechat_info, |s| {
                                    print_info_text.push_new_len(s);
                                }) {
                                    Ok(_) => Message::ShowUserInfoMessage(
                                        ShowUserInfoMessage::CommandFinished(Ok((
                                            wechat_info.key,
                                            wechat_info.account,
                                        ))),
                                    ),
                                    Err(e) => Message::ShowUserInfoMessage(
                                        ShowUserInfoMessage::CommandFinished(Err(e.to_string())),
                                    ),
                                }
                            }
                            Err(e) => Message::ShowUserInfoMessage(
                                ShowUserInfoMessage::CommandFinished(Err(e.to_string())),
                            ),
                        }
                    },
                    |msg| msg,
                )
            }
            ShowUserInfoMessage::CommandFinished(result) => {
                self.command_running = false;
                match result {
                    Ok((key, account)) => {
                        self.account = Some(account);
                        self.key = Some(key);
                        self.check_scroll()
                    }
                    Err(r) => {
                        self.print_info_text.push_new_err_len(r);
                        iced::Task::none()
                    }
                }
            }
            ShowUserInfoMessage::ButtonCopyKeyHex => 
                iced::clipboard::write(u8_to_string(&self.key.unwrap(), &"hex".to_string()).unwrap())
            ,
            ShowUserInfoMessage::ButtonCopyKeyBase64 => 
                iced::clipboard::write(u8_to_string(&self.key.unwrap(), &"base64".to_string()).unwrap())
            ,
        }
    }
    pub fn draw(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(5);
            col = col.push(self.print_info_text.draw().height(Length::Fill));
            if let Some(_) = &self.key {
                col = col.push(
                    Row::new()
                        .spacing(5)
                        .push(Space::with_width(10))
                        .push(
                            Button::new("复制key(hex)").on_press(Message::ShowUserInfoMessage(
                                ShowUserInfoMessage::ButtonCopyKeyHex,
                            )),
                        )
                        .push(Button::new("复制key(base64)").on_press(
                            Message::ShowUserInfoMessage(ShowUserInfoMessage::ButtonCopyKeyBase64),
                        )),
                )
            }
            col = col.push(Space::with_height(5));
            col
        })
    }
}
#[derive(Debug, Clone)]
pub enum ShowUserInfoMessage {
    UpdateWeChatInfo,
    CommandFinished(Result<([u8; 32], String), String>),
    ButtonCopyKeyHex,
    ButtonCopyKeyBase64,
}
