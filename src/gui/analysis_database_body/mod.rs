use std::{
    collections::{HashMap, LinkedList},
    fs::read_dir,
    path::Path,
};

use chrono::{DateTime, Datelike, Local, Timelike};
use cpal::Stream;
use diesel::SqliteConnection;
use iced::{
    Color, Length,
    widget::{
        Button, Column, Container, Image, Row, Scrollable, Space, Text, TextInput,
        scrollable::{AbsoluteOffset, Viewport},
    },
};
use iced_runtime::core::{image::Handle, text::Shaping};
use ollama_body::{OLLamaBody, OLLamaMessage};
use report_body::{AnalysisDatabaseReportMessage, ReportInfo, UserForSelect};

use self::module::module_macro_msg::{Contact, Session};

use super::{
    Message,
    config_body::ConfigBody,
    gui_util::{self, set_col_with_text, set_col_with_text_input},
};

mod module;
mod ollama_body;
mod report_body;
mod schema;

pub struct AnalysisDatabaseBody {
    decrypted_path: String,
    analysis_database_err_msg: Option<String>,
    conn: Option<Conn>,
    session_info: SessionInfo,
    msg_info: MsgInfo,
    report_info: ReportInfo,
    audio_stream: Option<(Stream, i64)>,
    body: Body,
    contact: HashMap<Option<String>, Contact>,
    emotion_map: HashMap<String, Result<ImageData, String>>,
    chat_room_user_list: HashMap<Option<String>, Vec<String>>,
    analysis_running: bool,
    msg_getting: bool,
    wechat_path: Option<String>,
    prev_body: LinkedList<PrevBody>,
    ollama_body: OLLamaBody,
}

#[derive(Clone)]
struct SessionInfo {
    session_list: Vec<Session>,
    display_session_list: Vec<Session>,
    session_scroll_id: iced::widget::scrollable::Id,
    absolute_offset: AbsoluteOffset,
    search_session: String,
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            session_list: vec![],
            display_session_list: vec![],
            session_scroll_id: iced::widget::scrollable::Id::new("analysis_session_scroll"),
            absolute_offset: AbsoluteOffset::default(),
            search_session: String::new(),
        }
    }
}

#[derive(Clone)]
struct MsgInfo {
    msg_list: Vec<(usize, module::module_msg::Msg)>,
    message_title: String,
    message_scroll_id: iced::widget::scrollable::Id,
    msg_page: usize,
    msg_page_input: String,
    audio_err_texts: HashMap<i64, String>,
    msg_str_usr_name: Option<Option<String>>,
    absolute_offset: AbsoluteOffset,
}

impl Default for MsgInfo {
    fn default() -> Self {
        Self {
            msg_list: vec![],
            message_title: String::new(),
            message_scroll_id: iced::widget::scrollable::Id::new("analysis_message_scroll"),
            msg_page: 0,
            msg_page_input: "1".to_string(),
            audio_err_texts: HashMap::new(),
            msg_str_usr_name: None,
            absolute_offset: AbsoluteOffset::default(),
        }
    }
}

enum ImageData {
    Image(Vec<u8>),
    Gif(iced_gif::Frames),
}

enum PrevBody {
    Session(SessionInfo),
    Msg(MsgInfo),
}

impl AnalysisDatabaseBody {
    pub fn new() -> Self {
        Self {
            decrypted_path: String::new(),
            analysis_database_err_msg: None,
            conn: None,
            body: Body::Session,
            contact: HashMap::new(),
            emotion_map: HashMap::new(),
            analysis_running: false,
            msg_getting: false,
            wechat_path: None,
            session_info: SessionInfo::default(),
            msg_info: MsgInfo::default(),
            audio_stream: None,
            prev_body: LinkedList::new(),
            report_info: ReportInfo::default(),
            chat_room_user_list: HashMap::new(),
            ollama_body: OLLamaBody::new(),
        }
    }
    fn get_user_name(&self, user: &Option<String>) -> UserForSelect {
        self.report_info.get_user_name(user, &self.contact)
    }
    pub fn update(
        &mut self,
        msg: AnalysisDatabaseMessage,
        config_body: &ConfigBody,
        theme: &iced::theme::Theme,
    ) -> iced::Task<Message> {
        match msg {
            AnalysisDatabaseMessage::UpdateAnalysisDatabase => {
                self.decrypted_path = config_body.decrypt_path.trim().to_owned();
                self.wechat_path = config_body.wechat_dir.to_owned();
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonAnalysis => {
                self.analysis_running = true;
                match self.get_conn() {
                    Ok(mut conn) => {
                        self.prev_body = LinkedList::new();
                        self.session_info = SessionInfo::default();
                        self.msg_info = MsgInfo::default();
                        match module::module_macro_msg::get_sessions(&mut conn.macro_msg_conn) {
                            Ok(v) => {
                                self.session_info.session_list = v;
                                self.session_info.display_session_list =
                                    self.session_info.session_list.clone();
                                self.body = Body::Session;
                                match module::module_macro_msg::get_contact(
                                    &mut conn.macro_msg_conn,
                                ) {
                                    Ok(map) => self.contact = map,
                                    Err(e) => {
                                        self.analysis_database_err_msg = Some(
                                            self.analysis_database_err_msg
                                                .as_ref()
                                                .map_or(e.to_string(), |s| {
                                                    format!("{};{}", s, e.to_string())
                                                }),
                                        )
                                    }
                                }
                                match module::module_macro_msg::get_chat_room_users_map(
                                    &mut conn.macro_msg_conn,
                                ) {
                                    Ok(map) => self.chat_room_user_list = map,
                                    Err(e) => {
                                        self.analysis_database_err_msg = Some(
                                            self.analysis_database_err_msg
                                                .as_ref()
                                                .map_or(e.to_string(), |s| {
                                                    format!("{};{}", s, e.to_string())
                                                }),
                                        )
                                    }
                                }
                            }
                            Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                        }
                        self.conn = Some(conn);
                    }
                    Err(r) => {
                        self.analysis_database_err_msg = Some(r.to_string());
                    }
                }
                self.analysis_running = false;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonSession(index, message_title) => {
                self.msg_getting = true;
                match module::module_msg::get_msg_by_user_name(
                    &self.session_info.display_session_list[index].str_usr_name,
                    &mut self.conn.as_mut().unwrap().msg_conn_map,
                ) {
                    Ok(s) => {
                        self.msg_info = MsgInfo::default();
                        self.msg_info.msg_list = s;
                        self.msg_info.msg_page = self.msg_info.msg_list.len() / 100;
                        self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                        self.msg_info.message_title = message_title;
                        self.msg_getting = false;
                        self.msg_info.msg_str_usr_name = Some(
                            self.session_info.display_session_list[index]
                                .str_usr_name
                                .clone(),
                        );
                        self.body = Body::Msg;
                        self.prev_body
                            .push_back(PrevBody::Session(self.session_info.clone()));
                        let mut command_vec = self.get_msg_command_by_msg_page();
                        command_vec.push(iced::widget::scrollable::snap_to::<Message>(
                            self.msg_info.message_scroll_id.clone(),
                            iced::widget::scrollable::RelativeOffset { x: 0.0, y: 1.0 },
                        ));
                        return iced::Task::batch(command_vec);
                    }
                    Err(e) => self.analysis_database_err_msg = Some(e.to_string()),
                }
                self.msg_getting = false;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonBack => {
                if let Some(prev_body) = self.prev_body.pop_back() {
                    match prev_body {
                        PrevBody::Session(session_info) => {
                            self.session_info = session_info;
                            self.body = Body::Session;
                            iced::widget::scrollable::scroll_to(
                                self.session_info.session_scroll_id.clone(),
                                self.session_info.absolute_offset.clone(),
                            )
                        }
                        PrevBody::Msg(msg_info) => {
                            self.msg_info = msg_info;
                            self.body = Body::Msg;
                            iced::widget::scrollable::scroll_to(
                                self.msg_info.message_scroll_id.clone(),
                                self.msg_info.absolute_offset.clone(),
                            )
                        }
                    }
                } else {
                    iced::Task::none()
                }
            }
            AnalysisDatabaseMessage::ButtonMsgPrev => {
                if self.msg_info.msg_page > 0 {
                    self.msg_info.msg_page -= 1;
                    self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                }
                iced::Task::batch(self.get_msg_command_by_msg_page())
            }
            AnalysisDatabaseMessage::ButtonMsgNext => {
                if self.msg_info.msg_page < self.msg_info.msg_list.len() / 100 {
                    self.msg_info.msg_page += 1;
                    self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                }
                iced::Task::batch(self.get_msg_command_by_msg_page())
            }
            AnalysisDatabaseMessage::InputMsgPage(s) => {
                self.msg_info.msg_page_input = s;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonMsgJumpTo(p) => {
                self.msg_info.msg_page = p;
                self.msg_info.msg_page_input = (self.msg_info.msg_page + 1).to_string();
                iced::Task::batch(self.get_msg_command_by_msg_page())
            }
            AnalysisDatabaseMessage::ButtonMsgPlayAudio(id, key) => {
                if let Some(conn) = self.conn.as_mut().unwrap().media_msg_conn_map.get_mut(&key) {
                    match gui_util::play_audio(|sample_rate| {
                        module::module_media_msg::get_audio_pcm(id, sample_rate, conn)
                    }) {
                        Ok(stream) => {
                            self.audio_stream = Some((stream, id));
                        }
                        Err(e) => {
                            self.msg_info.audio_err_texts.insert(id, e.to_string());
                        }
                    }
                } else {
                    self.msg_info
                        .audio_err_texts
                        .insert(id, "未找到音频数据库".to_string());
                }
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonMsgStopAudio => {
                self.audio_stream = None;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonTestSilkSdk => {
                module::module_media_msg::test_for_silk(
                    self.conn
                        .as_mut()
                        .unwrap()
                        .media_msg_conn_map
                        .get_mut(&0)
                        .unwrap(),
                )
                .unwrap();
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonAnalyzeReport => {
                self.report_info = ReportInfo::default();
                self.report_info.report_image = None;
                let report_data = match self.body {
                    Body::Session => {
                        self.prev_body
                            .push_back(PrevBody::Session(self.session_info.clone()));
                        module::module_msg::get_report_data(
                            &None,
                            &mut self.conn.as_mut().unwrap().msg_conn_map,
                        )
                    }
                    Body::Msg => {
                        self.prev_body
                            .push_back(PrevBody::Msg(self.msg_info.clone()));
                        module::module_msg::get_report_data(
                            &self.msg_info.msg_str_usr_name,
                            &mut self.conn.as_mut().unwrap().msg_conn_map,
                        )
                    }
                    _ => Err(anyhow::anyhow!("错误的页面")),
                };
                match report_data {
                    Ok(vec) => {
                        self.report_info.all_msg_count = vec.len();

                        macro_rules! build_report_data {
                                    ($([$ident:ident => ($x:expr),($y:expr),($z:expr)]),+ $(,)? ) => {
                                        $(let mut $ident = HashMap::new();)*
                                        for data in vec {
                                            let user = if data.is_sender.unwrap() == 1 {
                                                None
                                            } else {
                                                if let Some(data_extra) = data.bytes_extra {
                                                    if let Some(user_name) = data_extra.map.get(&1) {
                                                        Some(user_name[0].clone())
                                                    } else {
                                                        Some(data.str_talker.unwrap())
                                                    }
                                                } else {
                                                    Some(data.str_talker.unwrap())
                                                }
                                            };
                                            let date = DateTime::from_timestamp(data.create_time.unwrap() as i64, 0).unwrap();
                                            $(
                                                let f = $x;
                                                let key = f(&user,&date.into());
                                                if let Some(count) = $ident.get_mut(&key) {
                                                    *count += 1;
                                                } else {
                                                    $ident.insert(key, 1);
                                                }
                                            )*
                                        }
                                        $(
                                            self.report_info.$ident = $ident.iter()
                                                .map($y)
                                                .collect();
                                            self.report_info.$ident.sort_by($z);
                                        )*
                                    };
                                }

                        build_report_data!([
                            count_by_user => (
                                |user:&Option<String>,_date:&DateTime<Local>| user.clone()
                            ),(
                                |(k,v)| (self.get_user_name(k),*v)
                            ),(
                                |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_date_order_by_count => (
                                |_user,date:&DateTime<Local>| date.date_naive()
                            ),(
                                |(k,v)| (*k,*v)
                            ),(
                                |a,b| b.1.cmp(&a.1))
                            ],[
                                count_by_date_order_by_date => (
                                    |_user,date:&DateTime<Local>| date.date_naive()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_date_user => (
                                    |user:&Option<String>,date:&DateTime<Local>| (date.date_naive(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],[
                            count_by_date_month => (
                                    |_user,date:&DateTime<Local>| date.with_day(1).unwrap().date_naive()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| b.1.cmp(&a.1))
                            ],[
                            count_by_date_month_user => (
                                    |user:&Option<String>,date:&DateTime<Local>| (date.with_day(1).unwrap().date_naive(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| b.2.cmp(&a.2))
                            ],[
                            count_by_hour => (
                                    |_user,date:&DateTime<Local>| date.hour()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_hour_user => (
                                    |user:&Option<String>,date:&DateTime<Local>| (date.hour(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_day => (
                                    |_user,date:&DateTime<Local>| date.day()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_day_user => (
                                    |user:&Option<String>,date:&DateTime<Local>| (date.day(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_month => (
                                    |_user,date:&DateTime<Local>| date.month()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_month_user => (
                                    |user:&Option<String>,date:&DateTime<Local>| (date.month(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_year => (
                                    |_user,date:&DateTime<Local>| date.year()
                                ),(
                                    |(k,v)| (*k,*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                            count_by_year_user => (
                                    |user:&Option<String>,date:&DateTime<Local>| (date.year(),user.clone())
                                ),(
                                    |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                ),(
                                    |a,b| a.0.cmp(&b.0))
                            ],[
                                count_by_weekday => (
                                        |_user,date:&DateTime<Local>| date.weekday()
                                    ),(
                                        |(k,v)| (*k,*v)
                                    ),(
                                        |a,b| a.0.number_from_monday().cmp(&b.0.number_from_monday()))
                                ],[
                                count_by_weekday_user => (
                                        |user:&Option<String>,date:&DateTime<Local>| (date.weekday(),user.clone())
                                    ),(
                                        |((k1,k2),v)| (*k1,self.get_user_name(k2),*v)
                                    ),(
                                        |a,b| a.0.number_from_monday().cmp(&b.0.number_from_monday()))
                                ],
                        );
                        if self.report_info.count_by_date_order_by_date.len() != 0 {
                            self.report_info.count_by_date_start = self
                                .report_info
                                .count_by_date_order_by_date
                                .first()
                                .unwrap()
                                .0
                                .clone();
                            self.report_info.count_by_date_end = self
                                .report_info
                                .count_by_date_order_by_date
                                .last()
                                .unwrap()
                                .0
                                .clone();
                            self.report_info.count_by_date_top_start_date = self
                                .report_info
                                .count_by_date_order_by_date
                                .first()
                                .unwrap()
                                .0
                                .clone();
                            self.report_info.count_by_date_top_end_date = self
                                .report_info
                                .count_by_date_order_by_date
                                .last()
                                .unwrap()
                                .0
                                .clone();
                        }
                        for (user, _) in self.report_info.count_by_user.iter() {
                            self.report_info
                                .user_select_for_date_options
                                .push(user.clone());
                            self.report_info
                                .user_select_for_jieba_options
                                .push(user.clone());
                        }
                        self.report_info.err_info = None;
                    }
                    Err(e) => self.report_info.err_info = Some(e.to_string()),
                }
                self.body = Body::Report;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ScrollSession(viewport) => {
                self.session_info.absolute_offset = viewport.absolute_offset();
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ScrollMsg(viewport) => {
                self.msg_info.absolute_offset = viewport.absolute_offset();
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ButtonSearchSession => {
                self.msg_getting = true;
                self.session_info.search_session =
                    self.session_info.search_session.trim().to_owned();
                let check_contact = |contact: &Contact, search_session: &String| {
                    contact
                        .user_name
                        .as_ref()
                        .unwrap_or(&"".to_owned())
                        .contains(search_session)
                        || contact
                            .alias
                            .as_ref()
                            .unwrap_or(&"".to_owned())
                            .contains(search_session)
                        || contact
                            .nick_name
                            .as_ref()
                            .unwrap_or(&"".to_owned())
                            .contains(search_session)
                        || contact
                            .remark
                            .as_ref()
                            .unwrap_or(&"".to_owned())
                            .contains(search_session)
                };
                if self.session_info.search_session.len() == 0 {
                    self.session_info.display_session_list = self.session_info.session_list.clone();
                } else {
                    self.session_info.display_session_list = vec![];
                    for session in &self.session_info.session_list {
                        if let Some(contact) = self.contact.get(&session.str_usr_name) {
                            if check_contact(contact, &self.session_info.search_session) {
                                self.session_info.display_session_list.push(session.clone());
                            } else {
                                if contact.r#type == Some(2) {
                                    if let Some(user_list) =
                                        self.chat_room_user_list.get(&contact.user_name)
                                    {
                                        for user in user_list {
                                            if let Some(user_contact) =
                                                self.contact.get(&Some(user.to_owned()))
                                            {
                                                if check_contact(
                                                    user_contact,
                                                    &self.session_info.search_session,
                                                ) {
                                                    self.session_info
                                                        .display_session_list
                                                        .push(session.clone());
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                self.msg_getting = false;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::InputSearchSession(input) => {
                self.session_info.search_session = input;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::ReqwestGetEmotion((url, result)) => {
                let result = match result {
                    Ok(vec) => match iced_gif::Frames::from_bytes(vec.clone()) {
                        Ok(frames) => Ok(ImageData::Gif(frames)),
                        Err(_) => Ok(ImageData::Image(vec)),
                    },
                    Err(e) => Err(e),
                };
                self.emotion_map.insert(url, result);
                iced::Task::none()
            }
            AnalysisDatabaseMessage::OpenFile(path) => {
                crate::util::open_file(path).ok();
                iced::Task::none()
            }
            AnalysisDatabaseMessage::AnalysisDatabaseReportMessage(msg) => {
                self.report_info
                    .update(msg, &self.msg_info, &self.contact, theme)
            }
            AnalysisDatabaseMessage::ButtonOLLamaOpen => {
                match self.body {
                    Body::Session => {
                        self.prev_body
                            .push_back(PrevBody::Session(self.session_info.clone()));
                    }
                    Body::Msg => {
                        self.prev_body
                            .push_back(PrevBody::Msg(self.msg_info.clone()));
                    }
                    _ => (),
                };
                self.ollama_body.update_msg(
                    &self.msg_info.msg_list,
                    &self.contact,
                    &self.msg_info.message_title,
                );
                self.body = Body::OLLama;
                iced::Task::none()
            }
            AnalysisDatabaseMessage::OLLamaMessage(ollama_msg) => {
                self.ollama_body.update(ollama_msg)
            }
        }
    }

    fn get_conn(&self) -> Result<Conn, anyhow::Error> {
        let decrypted_path = Path::new(self.decrypted_path.trim());
        let mut msg_conn_map = HashMap::new();
        let mut media_msg_conn_map = HashMap::new();
        let mut macro_msg_conn = None;
        let mut emotion_conn = None;
        for entry in read_dir(decrypted_path)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("decrypted_MSG") && filename.ends_with(".db") {
                        if let Ok(index) = filename
                            .replace("decrypted_MSG", "")
                            .replace(".db", "")
                            .parse::<usize>()
                        {
                            msg_conn_map.insert(index, module::get_conn(entry.path().display())?);
                        }
                    } else if filename.starts_with("decrypted_MediaMSG")
                        && filename.ends_with(".db")
                    {
                        if let Ok(index) = filename
                            .replace("decrypted_MediaMSG", "")
                            .replace(".db", "")
                            .parse::<usize>()
                        {
                            media_msg_conn_map
                                .insert(index, module::get_conn(entry.path().display())?);
                        }
                    } else if filename == "decrypted_MicroMsg.db" {
                        macro_msg_conn = Some(module::get_conn(entry.path().display())?);
                    } else if filename == "decrypted_Emotion.db" {
                        emotion_conn = Some(module::get_conn(entry.path().display())?);
                    }
                }
            }
        }
        Ok(Conn {
            msg_conn_map,
            media_msg_conn_map,
            macro_msg_conn: macro_msg_conn.ok_or(anyhow::anyhow!("未找到MicroMsg数据库"))?,
            emotion_conn: emotion_conn.ok_or(anyhow::anyhow!("未找到Emotion数据库"))?,
        })
    }

    fn get_msg_by_msg_page(&self) -> &[(usize, module::module_msg::Msg)] {
        if (self.msg_info.msg_page + 1) * 100 > self.msg_info.msg_list.len() {
            &self.msg_info.msg_list[self.msg_info.msg_page * 100..]
        } else {
            &self.msg_info.msg_list
                [self.msg_info.msg_page * 100..(self.msg_info.msg_page + 1) * 100]
        }
    }

    fn get_msg_command_by_msg_page(&self) -> Vec<iced::Task<Message>> {
        let mut emotion_vec = vec![];
        for (_, msg) in self.get_msg_by_msg_page() {
            match msg.load_msg_data(&self.wechat_path) {
                module::module_msg::MsgData::Emotion(url) => {
                    if let Some(url) = url {
                        if self.emotion_map.get(&url).is_none() {
                            emotion_vec.push(url);
                        }
                    }
                }
                _ => {}
            }
        }
        let command_vec = emotion_vec
            .iter()
            .map(|url| {
                let url1 = url.clone();
                let url2 = url.clone();
                iced::Task::<Message>::perform(
                    async move {
                        let r = reqwest::get(url1).await?;
                        r.bytes().await.map(|byte| byte.to_vec())
                    },
                    move |r| {
                        Message::AnalysisDatabaseMessage(
                            AnalysisDatabaseMessage::ReqwestGetEmotion((
                                url2.clone(),
                                r.map_err(|e| e.to_string()),
                            )),
                        )
                    },
                )
            })
            .collect::<Vec<_>>();
        command_vec
    }
    pub fn check_command_running(&self) -> bool {
        self.ollama_body.check_command_running()
    }
    pub fn check_arc_data(&self) -> iced::Task<Message> {
        self.ollama_body.check_arc_data()
    }
    fn draw_message(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(10);
            let list = self.get_msg_by_msg_page();
            for (index, msg) in list {
                col = col.push({
                    let mut row = Row::new();
                    let container = Container::new({
                        let mut col = Column::new().spacing(3);
                        if let Some(text) = msg.get_user_name(&self.contact) {
                            col = col.push(Text::new(text).size(19).shaping(Shaping::Advanced));
                        }
                        col = col.push(
                                Space::with_height(3)
                            ).push(
                                match msg.load_msg_data(&self.wechat_path) {
                                    module::module_msg::MsgData::Text(s) => {
                                        Container::new(
                                            Text::new(s).size(18).shaping(Shaping::Advanced)
                                        )
                                    },
                                    module::module_msg::MsgData::Image(image) => {
                                        match image {
                                            Ok(image) => {
                                                Container::new(
                                                    Button::new(
                                                        Image::new(Handle::from_bytes(image.clone()))
                                                    )
                                                    .padding(0)
                                                    .on_press(Message::OpenImage(image))
                                                )
                                            },
                                            Err(err) => {
                                                Container::new(Text::new(format!("图片获取失败，错误信息： {}",err.to_string())).size(18))
                                            },
                                        }
                                    },
                                    module::module_msg::MsgData::Other(s) => {
                                        Container::new(
                                            Text::new(s).size(18).shaping(Shaping::Advanced)
                                        )
                                    },
                                    module::module_msg::MsgData::Audio(id) => {
                                        Container::new(
                                            {
                                                let mut col = Column::new();
                                                col = col.push(
                                                    {
                                                        let mut row = Row::new().spacing(5);
                                                        row = row.push(
                                                            Button::new("播放音频").on_press(
                                                                Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonMsgPlayAudio(id, *index))
                                                            )
                                                        );
                                                        if let Some((_,playing_id)) = self.audio_stream {
                                                            if playing_id == id {
                                                                row = row.push(
                                                                    Button::new("停止播放").on_press(
                                                                        Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::ButtonMsgStopAudio)
                                                                    )
                                                                )
                                                            }
                                                        }
                                                        row
                                                    }
                                                );
                                                if let Some(s) = self.msg_info.audio_err_texts.get(&id) {
                                                    col = col.push(
                                                        Text::new(s).style(|_| iced::widget::text::Style{
                                                            color: Some(Color::from_rgb(1.0, 0.0, 0.0))
                                                        })
                                                    )
                                                }
                                                col
                                            }
                                        )
                                    },
                                    module::module_msg::MsgData::Emotion(url) => {
                                        if let Some(url) = url {
                                            if let Some(data) = self.emotion_map.get(&url) {
                                                match data {
                                                    Ok(image_data) => {
                                                        match image_data {
                                                            ImageData::Image(image) => {
                                                                Container::new(Image::new(Handle::from_bytes(image.clone())))
                                                            },
                                                            ImageData::Gif(frames) => {
                                                                Container::new(iced_gif::Gif::new(frames))
                                                            },
                                                        }
                                                    },
                                                    Err(e) => {
                                                        Container::new(
                                                            Text::new(format!("表情包加载失败，错误原因：{}",e))
                                                        )
                                                    },
                                                }
                                            } else {
                                                Container::new(
                                                    Text::new("表情包加载中")
                                                )
                                            }
                                        } else {
                                            Container::new(
                                                Text::new("未找到表情包的地址")
                                            )
                                        }
                                    },
                                    module::module_msg::MsgData::Voip(str) => {
                                        Container::new(Text::new(str).shaping(Shaping::Advanced))
                                    },
                                    module::module_msg::MsgData::Video((image, video_path)) => {
                                        match image {
                                            Ok(image) => {
                                                Container::new(
                                                    Button::new(
                                                        Image::new(Handle::from_bytes(image))
                                                    ).padding(0)
                                                    .on_press_maybe(
                                                        if let Ok(Some(video_path)) = video_path {
                                                            if video_path.exists() || video_path.is_file() {
                                                                Some(Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::OpenFile(video_path.to_str().unwrap().to_string())))
                                                            } else {
                                                                None
                                                            }
                                                        } else {
                                                            None
                                                        }
                                                    )
                                                )
                                            },
                                            Err(e) => Container::new(Text::new(format!("视频预览获取失败：{}",e.to_string()))),
                                        }
                                    },
                                }
                            ).push(
                                Space::with_height(3)
                            );
                        col
                    }).width(700);
                    if msg.is_sender == Some(1) {
                        row = row
                            .push(Container::new(
                                {
                                    let mut col = Column::new();
                                    if let Some(create_time) = msg.create_time {
                                        col = col.push(
                                            Text::new(Into::<DateTime<Local>>::into(DateTime::from_timestamp(create_time as i64, 0).unwrap()).to_string())
                                        );
                                    }
                                    col
                                }
                            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Right))
                            .push(container.align_x(iced::alignment::Horizontal::Right).style(|_theme|
                                iced::widget::container::Style::default()
                                    .background(Color::from_rgb(0.2, 0.2, 0.8))
                                    .color(Color::from_rgb(0.8, 0.8, 0.2))
                                ))
                            .push(Space::with_width(20))
                        ;
                    } else {
                        row = row
                            .push(Space::with_width(10))
                            .push(container.align_x(iced::alignment::Horizontal::Left).style(|theme|
                                iced::widget::container::Style::default()
                                    .background(
                                        match theme {
                                            iced::Theme::Dark => Color::from_rgb(0.2, 0.2, 0.2),
                                            _ => Color::from_rgb(0.8, 0.8, 0.8)
                                        }
                                    )
                            ))
                            .push(Container::new(
                                {
                                    let mut col = Column::new();
                                    if let Some(create_time) = msg.create_time {
                                        col = col.push(
                                            Text::new(Into::<DateTime<Local>>::into(DateTime::from_timestamp(create_time as i64, 0).unwrap()).to_string())
                                        );
                                    }
                                    col
                                }
                            ).width(Length::Fill).align_x(iced::alignment::Horizontal::Left))
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
            for (index, session) in self.session_info.display_session_list.iter().enumerate() {
                let content = self.contact.get(&session.str_usr_name);
                let mut nick_name = session.str_nick_name.clone().unwrap_or("".to_owned());
                if let Some(content) = content {
                    if let Some(remark) = &content.remark {
                        if remark.len() != 0 {
                            nick_name = format!("{} ({})", nick_name, remark);
                        }
                    }
                };
                col = col.push(
                    Button::new(
                        Column::new()
                            .push(
                                Text::new(nick_name.clone())
                                    .size(20)
                                    .shaping(Shaping::Advanced),
                            )
                            .push(
                                Text::new(session.str_content.clone().unwrap_or("".to_owned()))
                                    .size(17)
                                    .shaping(Shaping::Advanced),
                            ),
                    )
                    .width(Length::Fill)
                    .style(|theme, status| iced::widget::button::secondary(theme, status))
                    .on_press_maybe(
                        if !self.analysis_running && !self.msg_getting {
                            Some(Message::AnalysisDatabaseMessage(
                                AnalysisDatabaseMessage::ButtonSession(index, nick_name.clone()),
                            ))
                        } else {
                            None
                        },
                    ),
                )
            }
            col
        })
    }

    pub fn draw(&self) -> Container<Message> {
        Container::new({
            let mut col = Column::new().spacing(5);
            match self.body {
                Body::Session => {
                    col = col.push(
                        Scrollable::new(self.draw_session().width(Length::Fill))
                            .height(Length::Fill)
                            .on_scroll(|viewport| {
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ScrollSession(viewport),
                                )
                            })
                            .id(self.session_info.session_scroll_id.clone()),
                    );
                    if self.session_info.session_list.len() != 0 {
                        col = set_col_with_text_input(
                            col,
                            "搜索",
                            |input| {
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::InputSearchSession(input),
                                )
                            },
                            if !self.analysis_running && !self.msg_getting {
                                Some(Into::<Message>::into(
                                    AnalysisDatabaseMessage::ButtonSearchSession,
                                ))
                            } else {
                                None
                            },
                            &self.session_info.search_session,
                            "搜索",
                            &None::<String>,
                        );
                    }
                }
                Body::Msg => {
                    col = col.push(
                        Text::new(&self.msg_info.message_title)
                            .size(20)
                            .shaping(Shaping::Advanced),
                    );
                    col = col.push(
                        Scrollable::new(self.draw_message().width(Length::Fill))
                            .height(Length::Fill)
                            .on_scroll(|viewport| {
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ScrollMsg(viewport),
                                )
                            })
                            .id(self.msg_info.message_scroll_id.clone()),
                    );
                    col = col.push({
                        let mut row = Row::new()
                            .spacing(5)
                            .push(Space::with_width(5))
                            .push(Button::new("上一页").on_press_maybe(
                                if self.msg_info.msg_page == 0 {
                                    None
                                } else {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonMsgPrev,
                                    ))
                                },
                            ))
                            .push(
                                TextInput::new("", &self.msg_info.msg_page_input)
                                    .width(45)
                                    .on_input(|s| {
                                        Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::InputMsgPage(s),
                                        )
                                    }),
                            )
                            .push(
                                Text::new(format!(
                                    "总页数：{}",
                                    self.msg_info.msg_list.len() / 100 + 1
                                ))
                                .height(Length::Shrink),
                            )
                            .push(Button::new("跳转").on_press_maybe({
                                if let Ok(p) = self.msg_info.msg_page_input.trim().parse::<usize>()
                                {
                                    if p < 1 || (p - 1) * 100 > self.msg_info.msg_list.len() {
                                        None
                                    } else {
                                        Some(Message::AnalysisDatabaseMessage(
                                            AnalysisDatabaseMessage::ButtonMsgJumpTo(p - 1),
                                        ))
                                    }
                                } else {
                                    None
                                }
                            }))
                            .push(Button::new("下一页").on_press_maybe(
                                if (self.msg_info.msg_page + 1) * 100 > self.msg_info.msg_list.len()
                                {
                                    None
                                } else {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonMsgNext,
                                    ))
                                },
                            ))
                            .push(Button::new("返回").on_press_maybe(
                                if !self.analysis_running && !self.msg_getting {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonBack,
                                    ))
                                } else {
                                    None
                                },
                            ))
                            .push(Button::new("数据图表").on_press_maybe(
                                if !self.analysis_running && !self.msg_getting {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonAnalyzeReport,
                                    ))
                                } else {
                                    None
                                },
                            ));
                        if let Body::Msg = self.body {
                            row = row.push(Button::new("OLLama").on_press_maybe(
                                if !self.analysis_running && !self.msg_getting {
                                    Some(Message::AnalysisDatabaseMessage(
                                        AnalysisDatabaseMessage::ButtonOLLamaOpen,
                                    ))
                                } else {
                                    None
                                },
                            ))
                        }
                        #[cfg(feature = "dev")]
                        {
                            row = row.push(Button::new("开发用：测试silk库").on_press(
                                Message::AnalysisDatabaseMessage(
                                    AnalysisDatabaseMessage::ButtonTestSilkSdk,
                                ),
                            ));
                        }
                        row
                    });
                }
                Body::Report => {
                    col = col.push(
                        Scrollable::new(self.report_info.draw().width(Length::Fill))
                            .height(Length::Fill),
                    );

                    col = col.push({
                        let row = Row::new().spacing(5).push(Space::with_width(5)).push(
                            Button::new("返回").on_press(Message::AnalysisDatabaseMessage(
                                AnalysisDatabaseMessage::ButtonBack,
                            )),
                        );
                        row
                    });
                }
                Body::OLLama => {
                    col = col
                        .push(
                            Scrollable::new(self.ollama_body.draw().width(Length::Fill))
                                .height(Length::Fill),
                        )
                        .push(Row::new().spacing(5).push(Space::with_width(5)).push(
                            Button::new("返回").on_press(Message::AnalysisDatabaseMessage(
                                AnalysisDatabaseMessage::ButtonBack,
                            )),
                        ));
                }
            }
            col = set_col_with_text(
                col,
                "解密文件夹",
                if !self.analysis_running && !self.msg_getting {
                    Some(Into::<Message>::into(
                        AnalysisDatabaseMessage::ButtonAnalysis,
                    ))
                } else {
                    None
                },
                &self.decrypted_path,
                "解析",
                &self.analysis_database_err_msg,
            );
            col = col.push(Space::with_height(5));
            col
        })
    }
}

#[derive(Debug, Clone)]
pub enum AnalysisDatabaseMessage {
    AnalysisDatabaseReportMessage(AnalysisDatabaseReportMessage),
    UpdateAnalysisDatabase,
    ButtonAnalysis,
    ButtonSession(usize, String),
    ButtonBack,
    ButtonTestSilkSdk,
    ButtonMsgPrev,
    ButtonMsgNext,
    InputMsgPage(String),
    ButtonMsgJumpTo(usize),
    ButtonMsgPlayAudio(i64, usize),
    ButtonMsgStopAudio,
    ButtonAnalyzeReport,
    ButtonSearchSession,
    InputSearchSession(String),
    ScrollSession(Viewport),
    ScrollMsg(Viewport),
    ReqwestGetEmotion((String, Result<Vec<u8>, String>)),
    OpenFile(String),
    ButtonOLLamaOpen,
    OLLamaMessage(OLLamaMessage),
}

impl Into<Message> for AnalysisDatabaseMessage {
    fn into(self) -> Message {
        Message::AnalysisDatabaseMessage(self)
    }
}

struct Conn {
    msg_conn_map: HashMap<usize, SqliteConnection>,
    macro_msg_conn: SqliteConnection,
    media_msg_conn_map: HashMap<usize, SqliteConnection>,
    emotion_conn: SqliteConnection,
}

#[derive(PartialEq, Eq)]
enum Body {
    Session,
    Msg,
    Report,
    OLLama,
}
