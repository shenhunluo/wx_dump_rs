use std::sync::{Arc, RwLock};

use iced::{
    widget::{Button, Column, Container, Row, Space},
    Length,
};
use tokio::sync::watch::{Receiver, Sender};

use crate::action::get_database::get_database;

use super::{
    config_body::ConfigBody,
    gui_util::{set_col_with_text, set_col_with_text_input, PrintInfoText},
    show_user_info_body::ShowUserInfoBody,
    Message,
};

pub struct GetDatabaseBody {
    wechat_path: Option<String>,
    save_path: String,
    account: String,
    select_dir: String,
    need_select_dir: Arc<RwLock<bool>>,
    select_dir_index_rx: Receiver<usize>,
    select_dir_index_tx: Sender<usize>,
    command_running: bool,
    print_info_text: Arc<PrintInfoText>,
}

impl GetDatabaseBody {
    pub fn new() -> GetDatabaseBody {
        let (usize_tx, usize_rx) = tokio::sync::watch::channel(0);
        Self {
            save_path: String::new(),
            account: String::new(),
            select_dir: "1".to_string(),
            command_running: false,
            select_dir_index_rx: usize_rx,
            select_dir_index_tx: usize_tx,
            need_select_dir: Arc::new(RwLock::new(false)),
            wechat_path: None,
            print_info_text: Arc::new(PrintInfoText::new(iced::widget::scrollable::Id::new(
                "get_database_print_info",
            ))),
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
        msg: GetDatabaseMessage,
        config: &ConfigBody,
        show_user_info_body: &ShowUserInfoBody,
    ) -> iced::Task<Message> {
        match msg {
            GetDatabaseMessage::ButtonStart => {
                self.command_running = true;
                let need_select_dir = self.need_select_dir.clone();
                let rx = self.select_dir_index_rx.clone();
                let wechat_path = self.wechat_path.as_ref().map(|s| s.trim().to_owned());
                let save_dir = self.save_path.trim().to_owned();
                let account = self.account.trim().to_owned();
                let print_info = self.print_info_text.clone();
                let key = Arc::new(RwLock::new(vec![0]));
                iced::Task::perform(
                    async move {
                        let r: Result<(), anyhow::Error> = get_database(
                            &wechat_path,
                            &save_dir,
                            &account,
                            |s| print_info.push_new_len(s),
                            |mut keys| {
                                key.write().unwrap().append(&mut keys);
                                async {
                                    *(need_select_dir.write().unwrap()) = true;
                                    let mut rx = rx.clone();
                                    loop {
                                        print_info.push_new_len("请输入编号:");
                                        let r =
                                            rx.changed().await.map(|_| *rx.borrow_and_update())?;
                                        if r == 0 {
                                            println!("111");
                                            continue;
                                        } else {
                                            if key.read().unwrap().contains(&r) {
                                                *(need_select_dir.write().unwrap()) = false;
                                                print_info.push_to_last(r);
                                                break Ok::<_, anyhow::Error>(r);
                                            } else {
                                                print_info.push_new_err_len("请输入正确的编号");
                                                continue;
                                            }
                                        }
                                    }
                                }
                            },
                        )
                        .await;
                        Message::GetDatabaseMessage(GetDatabaseMessage::CommandFinished(
                            r.map_err(|e| e.to_string()),
                        ))
                    },
                    |msg| msg,
                )
            }
            GetDatabaseMessage::UpdateGetDatabase => {
                self.save_path = config.save_path.clone();
                self.wechat_path = config.wechat_dir.clone();
                if let Some(account) = show_user_info_body.get_account() {
                    self.account = account.clone();
                }
                iced::Task::none()
            }
            GetDatabaseMessage::InputAccount(s) => {
                self.account = s;
                iced::Task::none()
            }
            GetDatabaseMessage::InputSelectDir(s) => {
                if s.len() == 0 {
                    self.select_dir = s;
                } else {
                    if let Ok(select_dir) = s.parse::<usize>() {
                        self.select_dir = select_dir.to_string();
                    }
                }
                iced::Task::none()
            }
            GetDatabaseMessage::ButtonSelectDir => {
                self.select_dir_index_tx
                    .send(self.select_dir.parse().unwrap())
                    .unwrap();
                iced::Task::none()
            }
            GetDatabaseMessage::CommandFinished(r) => {
                self.command_running = false;
                self.select_dir_index_tx.send(0).unwrap();
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
                &Some("注意：备份操作将清空该文件夹！！！"),
            );
            col = set_col_with_text(
                col,
                "微信文件夹",
                None::<Message>,
                self.wechat_path
                    .as_ref()
                    .unwrap_or(&"默认文件夹".to_owned()),
                "",
                &None::<String>,
            );
            col = set_col_with_text_input(
                col,
                "用户名",
                |s| GetDatabaseMessage::InputAccount(s),
                None,
                &self.account,
                "",
                &None::<String>,
            );
            col = col.push(self.print_info_text.draw().height(Length::Fill));
            if *self.need_select_dir.read().unwrap() {
                col = set_col_with_text_input(
                    col,
                    "请选择",
                    |s| GetDatabaseMessage::InputSelectDir(s),
                    self.select_dir
                        .parse::<usize>()
                        .ok()
                        .map(|_| GetDatabaseMessage::ButtonSelectDir),
                    &self.select_dir,
                    "确定",
                    &None::<String>,
                );
            }
            col = col.push(Row::new().spacing(5).push(Space::with_width(10)).push(
                if self.command_running {
                    Button::new("备份运行中")
                } else {
                    Button::new("开始备份")
                        .on_press(Message::GetDatabaseMessage(GetDatabaseMessage::ButtonStart))
                },
            ));
            col = col.push(Space::with_height(5));
            col
        })
    }
}

#[derive(Debug, Clone)]
pub enum GetDatabaseMessage {
    UpdateGetDatabase,
    ButtonStart,
    InputAccount(String),
    InputSelectDir(String),
    ButtonSelectDir,
    CommandFinished(Result<(), String>),
}

impl Into<Message> for GetDatabaseMessage {
    fn into(self) -> Message {
        Message::GetDatabaseMessage(self)
    }
}
