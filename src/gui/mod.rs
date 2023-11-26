use iced::{Application, widget::{Column, Space, Row, Button}, Length, Subscription};

use self::{config_body::{ConfigMessage, ConfigBody}, show_user_info_body::{ShowUserInfoMessage, ShowUserInfoBody}, get_database_body::{GetDatabaseBody, GetDatabaseMessage}, decrypt_body::{DecryptMessage, DecryptBody}, analysis_database_body::{AnalysisDatabaseMessage, AnalysisDatabaseBody}};

mod config_body;
mod show_user_info_body;
mod get_database_body;
mod gui_util;
mod decrypt_body;
mod analysis_database_body;

pub struct WxDumpGui {
    body: Body,
    config_body: ConfigBody,
    show_user_info_body: ShowUserInfoBody,
    get_database_body: GetDatabaseBody,
    decrypt_body: DecryptBody,
    analysis_database_body: AnalysisDatabaseBody,
}

impl Application for WxDumpGui {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::theme::Theme;

    type Flags = super::Flags;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            WxDumpGui {
                body: Body::Config,
                config_body: ConfigBody::new(),
                show_user_info_body: ShowUserInfoBody::new(),
                get_database_body: GetDatabaseBody::new(),
                decrypt_body: DecryptBody::new(),
                analysis_database_body: AnalysisDatabaseBody::new(),
            },
            iced::Command::<Message>::none(),
        )
    }

    fn title(&self) -> String {
        "微信记录解密查看器".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ConfigMessage(msg) => {
                self.config_body.update(msg)
            },
            Message::ButtonConfig => self.body = Body::Config,
            Message::ButtonShowUserInfo => {
                let r = self.show_user_info_body.update(ShowUserInfoMessage::UpdateWeChatInfo, &self.config_body);
                self.body = Body::ShowUserInfo;
                return r;
            },
            Message::ButtonGetDatabase => {
                let r = self.get_database_body.update(GetDatabaseMessage::UpdateGetDatabase, &self.config_body, &self.show_user_info_body);
                self.body = Body::GetDatabase;
                return r;
            },
            Message::GetDatabaseMessage(msg) => return self.get_database_body.update(msg, &self.config_body, &self.show_user_info_body),
            Message::ShowUserInfoMessage(msg) => {
                return self.show_user_info_body.update(msg, &self.config_body);
            },
            Message::CheckArcData => {
                match self.body {
                    Body::Config => todo!(),
                    Body::ShowUserInfo => return self.show_user_info_body.check_scroll(),
                    Body::GetDatabase => return self.get_database_body.check_scroll(),
                    Body::Decrypt => return self.decrypt_body.check_scroll(),
                    Body::AnalysisDatabase => todo!(),
                }
            },
            Message::DecryptMessage(msg) => {
                return self.decrypt_body.update(msg, &self.config_body, &self.show_user_info_body);
            },
            Message::ButtonDecrypt => {
                let r = self.decrypt_body.update(DecryptMessage::UpdateDecrypt, &self.config_body, &self.show_user_info_body);
                self.body = Body::Decrypt;
                return r;
            },
            Message::ButtonAnalysisDatabase => {
                let r = self.analysis_database_body.update(AnalysisDatabaseMessage::UpdateAnalysisDatabase, &self.config_body);
                self.body = Body::AnalysisDatabase;
                return r;
            },
            Message::AnalysisDatabaseMessage(msg) => return self.analysis_database_body.update(msg, &self.config_body),
        }
        iced::Command::<Message>::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.body {
            Body::Config => iced::subscription::Subscription::none(),
            Body::ShowUserInfo => if self.show_user_info_body.check_command_running() {
                iced::time::every(std::time::Duration::from_millis(10)).map(|_| Message::CheckArcData)
            } else {
                iced::subscription::Subscription::none()
            },
            Body::GetDatabase => if self.get_database_body.check_command_running() {
                iced::time::every(std::time::Duration::from_millis(10)).map(|_| Message::CheckArcData)
            } else {
                iced::subscription::Subscription::none()
            },
            Body::Decrypt => if self.decrypt_body.check_command_running() {
                iced::time::every(std::time::Duration::from_millis(10)).map(|_| Message::CheckArcData)
            } else {
                iced::subscription::Subscription::none()
            },
            Body::AnalysisDatabase => iced::subscription::Subscription::none(),
        }
    }
    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Column::new()
            .push(Space::with_height(10))
            .push(
                match self.body {
                    Body::Config => {
                        self.config_body.draw()
                    },
                    Body::ShowUserInfo => self.show_user_info_body.draw(),
                    Body::GetDatabase => self.get_database_body.draw(),
                    Body::Decrypt => self.decrypt_body.draw(),
                    Body::AnalysisDatabase => self.analysis_database_body.draw(),
                }.height(Length::Fill)
            )
            .push(
                Row::new()
                    .spacing(5.0)
                    .height(Length::Fixed(40.0))
                    .push(
                        Space::with_width(Length::Fixed(5.0))
                    ).push(
                        Button::new("配置").on_press(Message::ButtonConfig)
                    ).push(
                        Button::new("查看用户信息").on_press_maybe(
                            if (self.config_body.process_id.is_some() || self.config_body.process_name.trim().len() != 0) && self.config_body.module_name.trim().len() != 0 {
                                Some(Message::ButtonShowUserInfo)
                            } else {
                                None
                            }
                        )
                    )
                    .push(
                        Button::new("备份数据库").on_press_maybe(
                            if self.config_body.save_path.trim().len() != 0 && !self.decrypt_body.check_command_running() {
                                Some(Message::ButtonGetDatabase)
                            } else {
                                None
                            }
                        )
                    )
                    .push(
                        Button::new("解密").on_press_maybe(
                            if self.config_body.save_path.trim().len() != 0 && self.config_body.decrypt_path.trim().len() != 0 && !self.get_database_body.check_command_running() {
                                Some(Message::ButtonDecrypt)
                            } else {
                                None
                            }
                        )
                    ).push(
                        Button::new("查看微信消息").on_press_maybe(
                            if !self.decrypt_body.check_command_running() && self.config_body.decrypt_path.trim().len() != 0 {
                                Some(Message::ButtonAnalysisDatabase)
                            } else {
                                None
                            }
                        )
                    )
            )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonConfig,
    ConfigMessage(ConfigMessage),
    ShowUserInfoMessage(ShowUserInfoMessage),
    ButtonShowUserInfo,
    ButtonGetDatabase,
    GetDatabaseMessage(GetDatabaseMessage),
    CheckArcData,
    ButtonDecrypt,
    DecryptMessage(DecryptMessage),
    ButtonAnalysisDatabase,
    AnalysisDatabaseMessage(AnalysisDatabaseMessage)

}

enum Body {
    Config,
    ShowUserInfo,
    GetDatabase,
    Decrypt,
    AnalysisDatabase,
}