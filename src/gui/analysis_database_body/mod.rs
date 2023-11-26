use std::{path::Path, fs::read_dir, collections::HashMap};

use diesel::SqliteConnection;
use iced::{widget::{Container, Space, Column, Text, Button, Scrollable, Row}, Length, Color};
use iced_runtime::core::text::Shaping;

use self::module::module_macro_msg::{Session, Contact};

use super::{Message, config_body::ConfigBody, gui_util::set_col_with_text};

mod schema;
mod module;

pub struct AnalysisDatabaseBody {
    decrypted_path: String,
    analysis_database_err_msg: Option<String>,
    conn: Option<Conn>,
    session_list: Vec<Session>,
    body: Body,
    contact: HashMap<Option<String>, Contact>,
    analysis_running: bool,
    msg_getting: bool,
    msg_list: Vec<module::module_msg::Msg>,
    message_title: String,
    message_scroll_id: iced::widget::scrollable::Id,
}

impl AnalysisDatabaseBody {
    pub fn new() -> Self {
        Self {
            decrypted_path: String::new(),
            analysis_database_err_msg: None,
            conn: None,
            session_list: vec![],
            body: Body::Session,
            contact: HashMap::new(),
            analysis_running: false,
            msg_getting: false,
            msg_list: vec![],
            message_title: String::new(),
            message_scroll_id: iced::widget::scrollable::Id::new("analysis_message_scroll"),
        }
    }

    pub fn update(&mut self, msg: AnalysisDatabaseMessage, config_body: &ConfigBody) -> iced::Command<Message> {
        match msg {
            AnalysisDatabaseMessage::UpdateAnalysisDatabase => {
                self.decrypted_path = config_body.decrypt_path.trim().to_owned();
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonAnalysis => {
                self.analysis_running = true;
                match self.get_conn() {
                    Ok(mut conn) => {
                        match module::module_macro_msg::get_sessions(&mut conn.macro_msg_conn) {
                            Ok(v) => {
                                self.session_list = v;
                                self.body = Body::Session;
                                match module::module_macro_msg::get_contact(&mut conn.macro_msg_conn) {
                                    Ok(map) => self.contact = map,
                                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                                } 
                            },
                            Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                        }
                        self.conn = Some(conn);
                    },
                    Err(r) => {
                        self.analysis_database_err_msg = Some(r.to_string());
                    },
                }
                self.analysis_running = false;
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonSession(index, message_title) => {
                self.msg_getting = true;
                match module::module_msg::get_msg_by_user_name(&self.session_list[index].str_usr_name, &mut self.conn.as_mut().unwrap().msg_conn_map) {
                    Ok(s) => {
                        self.msg_list = s;
                        self.message_title = message_title;
                        self.body = Body::Msg;
                        self.msg_getting = false;
                        return iced::widget::scrollable::snap_to::<Message>(self.message_scroll_id.clone(), iced::widget::scrollable::RelativeOffset{ x: 0.0, y: 1.0 });
                    },
                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                }
                self.msg_getting = false;
                iced::Command::none()
            },
            AnalysisDatabaseMessage::ButtonBack => {
                self.msg_list = vec![];
                self.body = Body::Session;
                iced::Command::none()
            },
        }
    }

    fn get_conn(&self) -> Result<Conn,anyhow::Error> {
        let decrypted_path = Path::new(self.decrypted_path.trim());
        let mut map = HashMap::new();
        let mut macro_msg_conn = None;
        for entry in read_dir(decrypted_path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("decrypted_MSG") && filename.ends_with(".db") {
                        if let Ok(index) = filename.replace("decrypted_MSG", "").replace(".db", "").parse::<usize>() {
                            map.insert(index, module::get_conn(entry.path().display())?);
                        }
                    } else if filename == "decrypted_MicroMsg.db" {
                        macro_msg_conn = Some(module::get_conn(entry.path().display())?);
                    }
                }
            }
        }
        Ok(Conn { msg_conn_map: map, macro_msg_conn: macro_msg_conn.ok_or(anyhow::anyhow!("未找到MicroMsg数据库"))? })
    }
    fn draw_message(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(10);
            for msg in &self.msg_list {
                col = col.push({
                    let mut row = Row::new();
                    let container = Container::new(
                        Column::new().push(
                            Text::new(msg.str_content.clone().unwrap_or("".to_owned())).size(18).shaping(Shaping::Advanced)
                        )
                    ).width(700);
                    if msg.is_sender == Some(1) {
                        row = row
                            .push(Space::with_width(Length::Fill))
                            .push(container.align_x(iced::alignment::Horizontal::Right).style(iced::theme::Container::Custom(Box::new(MsgContainerTheme::Right))))
                            .push(Space::with_width(20))
                        ;
                    } else {
                        row = row
                            .push(Space::with_width(10))
                            .push(container.align_x(iced::alignment::Horizontal::Left).style(iced::theme::Container::Custom(Box::new(MsgContainerTheme::Left))))
                            .push(Space::with_width(Length::Fill))
                        ;
                    }
                    row.width(Length::Fill)
                }).width(Length::Fill)
            }
            col
        })
    }
    fn draw_session(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new();
            for (index,session) in self.session_list.iter().enumerate() {
                let content = self.contact.get(&session.str_usr_name);
                let mut nick_name = session.str_nick_name.clone().unwrap_or("".to_owned());
                if let Some(content) = content {
                    if let Some(remark) = &content.remark {
                        if remark.len() != 0 {
                            nick_name = format!("{} ({})",nick_name,remark);
                        }
                    }
                };
                col = col.push(
                    Button::new(
                        Column::new()
                            .push(
                                Text::new(nick_name.clone()).size(20).shaping(Shaping::Advanced)
                            ).push(
                                Text::new(session.str_content.clone().unwrap_or("".to_owned())).size(17).shaping(Shaping::Advanced)
                            )
                    ).width(Length::Fill).style(iced::theme::Button::Secondary).on_press_maybe(
                        if !self.analysis_running && !self.msg_getting {
                            Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonSession(index,nick_name.clone())))
                        } else {
                            None
                        }
                    )
                )
            }
            col
        })
    }

    pub fn draw(&self) -> Container<Message> {
        Container::new(
            {
                let mut col = Column::new().spacing(5);
                match self.body {
                    Body::Session => {
                        col = col.push(
                            Scrollable::new(
                                self.draw_session().width(Length::Fill)
                            ).height(Length::Fill)
                        );
                    },
                    Body::Msg => {
                        col = col.push(
                            Text::new(&self.message_title).size(20).shaping(Shaping::Advanced)
                        );
                        col = col.push(
                            Scrollable::new(
                                self.draw_message().width(Length::Fill)
                            ).height(Length::Fill).id(self.message_scroll_id.clone())
                        );
                        col = col.push(
                            Row::new().spacing(5).push(Space::with_width(5)).push(
                                Button::new("返回").on_press_maybe(
                                    if  !self.analysis_running && !self.msg_getting {
                                        Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonBack))
                                    } else {
                                        None
                                    }
                                )
                            )
                        );
                    },
                }
                col = set_col_with_text(
                    col, 
                    "解密文件夹", 
                    if !self.analysis_running && !self.msg_getting {
                        Some(Into::<Message>::into(AnalysisDatabaseMessage::ButtonAnalysis))
                    } else {
                        None
                    }, 
                    &self.decrypted_path, 
                    "解析", 
                    &self.analysis_database_err_msg
                );
                col = col.push(Space::with_height(5));
                col
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum AnalysisDatabaseMessage {
    UpdateAnalysisDatabase,
    ButtonAnalysis,
    ButtonSession(usize, String),
    ButtonBack,
}

impl Into<Message> for AnalysisDatabaseMessage {
    fn into(self) -> Message {
        Message::AnalysisDatabaseMessage(self)
    }
}

struct Conn {
    msg_conn_map: HashMap<usize, SqliteConnection>,
    macro_msg_conn: SqliteConnection,
}

#[derive(PartialEq,Eq)]
enum Body {
    Session,
    Msg,
}

enum MsgContainerTheme {
    Left,
    Right,
}

impl iced::widget::container::StyleSheet for MsgContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        let mut s = style.appearance(&iced::theme::Container::Box);
        match self {
            MsgContainerTheme::Left => {
                s.background = Some(iced::Background::Color(Color::from_rgb(0.8, 0.8, 0.8)))
            },
            MsgContainerTheme::Right => {
                s.background = Some(iced::Background::Color(Color::from_rgb(0.2, 0.2, 0.8)))
            }
        }
        s
    }
}