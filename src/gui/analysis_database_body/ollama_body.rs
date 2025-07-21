use std::{collections::HashMap, sync::Arc, u64};

use crate::gui::{gui_util::{set_col_with_text, set_col_with_text_input}, Message};
use anyhow::Ok;
use chrono::{DateTime, Local};
use iced::{
    Length,
    widget::{Column, Container, Row, Space, Text, text::Shaping},
};
use rig::{completion::Chat, providers::ollama::Client as OllamaClient, vector_store::in_memory_store::InMemoryVectorStore};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use rig::client::{EmbeddingsClient,CompletionClient};
use super::{
    AnalysisDatabaseMessage,
    module::{module_macro_msg::Contact, module_msg::Msg},
};

pub struct OLLamaBody {
    msg: Vec<String>,
    base_url: String,
    l_l_model: String,
    rag_model: String,
    ask_str: String,
    contents: Vec<(Roles, String)>,
    task_chat_running: bool,
    task_rag_running: bool,
    response_lock: Arc<RwLock<String>>,
    rag_data: Option<InMemoryVectorStore<String>>,
    rag_building_err_msg: Option<String>,
    ask_err_msg: Option<String>,
}

enum Roles {
    User,
    Assistant,
}

impl OLLamaBody {
    pub fn new() -> Self {
        OLLamaBody {
            msg: vec![],
            base_url: "http://127.0.0.1:11434".to_owned(),
            l_l_model: "deepseek-r1:14b".to_owned(),
            rag_model: "nomic-embed-text".to_owned(),
            ask_str: String::new(),
            contents: vec![],
            task_chat_running: false,
            task_rag_running: false,
            response_lock: Arc::new(RwLock::new("".to_owned())),
            rag_data: None,
            rag_building_err_msg: None,
            ask_err_msg: None,
        }
    }
    pub fn update_msg(
        &mut self,
        msg_list: &Vec<(usize, Msg)>,
        contact: &HashMap<Option<String>, Contact>,
        msg_title: &String,
    ) {
        let mut vec = vec![];
        for (_, msg) in msg_list {
            if msg.r#type == Some(1) {
                let message = msg.str_content.clone().unwrap_or("".to_string());
                let timestamp = Into::<DateTime<Local>>::into(
                    DateTime::from_timestamp(msg.create_time.unwrap_or_default() as i64, 0)
                        .unwrap(),
                )
                .to_string();
                let user = if msg.is_sender == Some(1) {
                    "我".to_owned()
                } else {
                    if let Some(text) = msg.get_user_name(contact) {
                        text
                    } else {
                        msg_title.clone()
                    }
                };
                vec.push(format!("[{}] {}: {}",timestamp,user,message));
            }
        }
        if self.msg != vec {
            self.msg = vec;
            self.rag_data = None;
        }
    }
    pub fn update(&mut self, msg: OLLamaMessage) -> iced::Task<Message> {
        match msg {
            OLLamaMessage::InputBaseUrl(s) => {
                self.base_url = s;
                iced::Task::none()
            }
            OLLamaMessage::InputLLModel(s) => {
                self.l_l_model = s;
                iced::Task::none()
            }
            OLLamaMessage::InputRAGModel(s) => {
                self.rag_model = s;
                iced::Task::none()
            }
            OLLamaMessage::ButtonGetRAGData => {
                self.task_rag_running = true;
                let base_url = self.base_url.clone();
                let rag_model = self.rag_model.clone();
                let msg = self.msg.clone();
                iced::Task::perform(
                    async move {
                        // let r: Result<Vec<Vec<f32>>, anyhow::Error> = {
                        //     let client = ClientBuilder::new().build()?;
                        //     let resp = client
                        //         .post(format!("{}/api/embed", base_url))
                        //         .body(serde_json::to_string(&OLLamaRAGRequest::new(rag_model, msg))?)
                        //         .send()
                        //         .await?;
                        //     let resp = resp.error_for_status()?;
                        //     let resp = serde_json::from_str::<OLLamaRAGResponse>(&resp.text().await?)?;
                        //     Ok(resp.embeddings)
                        // };
                        let r: Result<InMemoryVectorStore<String>, anyhow::Error> = {
                            let client = OllamaClient::from_url(&base_url);
                            let embedding_model = client.embedding_model(&rag_model);  
                            let embeddings = client.embeddings(&embedding_model.model).documents(vec![msg.join("\n\n")])?.build().await?;
                            let vector_store = InMemoryVectorStore::from_documents(embeddings);
                            // let index = vector_store.index(embedding_model);
                            Ok(vector_store)
                        };
                        r
                    },
                    |r| {
                        OLLamaMessage::TaskDone(TaskResult::RAGTaskDone(r.map(|r| r.into()).map_err(|e| e.to_string()))).into()
                    }
                )
            }
            OLLamaMessage::InputAsk(s) => {
                self.ask_str = s;
                iced::Task::none()
            }
            OLLamaMessage::ButtonAsk => {
                self.contents.push((Roles::User, self.ask_str.clone()));
                self.contents.push((Roles::Assistant, String::new()));
                
                let messages = self.contents[..self.contents.len() - 2]
                    .iter()
                    .map(|(r, s)| match r {
                        Roles::User => rig::completion::message::Message::user(s),
                        Roles::Assistant => rig::completion::message::Message::assistant(s),
                    })
                    .collect::<Vec<_>>();
                // messages.push(OLLamaRequestMessage{
                //     role: "user".to_owned(),
                //     content: format!("参考聊天记录回答问题\n1.聊天记录是json格式的RAG数据，原始数据的格式是 [时间] 用户: 聊天内容 \n2.请记住聊天记录中所有细节\n3.一边读取聊天记录,一遍思考问题\n4.问题是: {}\n5.聊天记录:{}\n", self.ask_str,serde_json::to_string(self.rag_data.as_ref().unwrap()).unwrap()),
                // });
                let ask = self.ask_str.clone();
                self.ask_str = String::new();
                let response_lock = self.response_lock.clone();
                let base_url = self.base_url.clone();
                let l_l_model = self.l_l_model.clone();
                let rag_model = self.rag_model.clone();
                let vector_store = self.rag_data.as_ref().unwrap().clone();
                self.task_chat_running = true;
                iced::Task::perform(
                    async move {
                        let r: Result<(), anyhow::Error> = {
                            // let client = ClientBuilder::new().build()?;
                            // let resp = client
                            //     .post(format!("{}/api/chat", base_url))
                            //     .body(serde_json::to_string(&OLLamaChatRequest::new(l_l_model, messages))?)
                            //     .send()
                            //     .await?;
                            // let resp = resp.error_for_status()?;
                            // let mut resp_stream = resp.bytes_stream();
                            // while let Some(item) = resp_stream.next().await {
                            //     let item = String::from_utf8(item?.to_vec())?;
                            //     for line in item.lines() {
                            //         let resp = serde_json::from_str::<OLLamaChatResponse>(line)?;
                            //         let mut write_lock = response_lock.write().await;
                            //         write_lock.push_str(&resp.message.content);
                            //     }
                            // }
                            let client = OllamaClient::from_url(&base_url);
                            let embedding_model = client.embedding_model(&rag_model);
                            let index = vector_store.index(embedding_model);
                            println!("begin");
                            let agent = client
                                .agent(&l_l_model)
                                .preamble("你是一个问答助手")
                                .dynamic_context(1, index)
                                .max_tokens(32768)
                                .build();
                            let r = agent.chat(ask, messages).await?;
                            let mut write_lock = response_lock.write().await;
                            println!("resp :{r}");
                            write_lock.push_str(&r);
                            Ok(())
                        };
                        println!("r: {r:?}");
                        r
                    },
                    |r| OLLamaMessage::TaskDone(TaskResult::ChatTaskDone(r.map_err(|e| e.to_string()))).into(),
                )
            }
            OLLamaMessage::TaskDone(r) => {
                match r {
                    TaskResult::ChatTaskDone(r) => {
                        self.task_chat_running = false;
                        if let Err(e) = r {
                            self.ask_err_msg = Some(e);
                        } else {
                            self.ask_err_msg = None;
                        }
                        self.response_lock.blocking_write().clear();
                    },
                    TaskResult::RAGTaskDone(r) => {
                        self.task_rag_running = false;
                        match r {
                            Result::Ok(r) => {
                                self.rag_data = Some(r);
                                self.rag_building_err_msg = None;
                            },
                            Err(e) => {
                                self.rag_building_err_msg = Some(e);
                            },
                        }
                    },
                }
                iced::Task::none()
            }
            OLLamaMessage::TaskSetContent(s) => {
                let len = self.contents.len() - 1;
                self.contents[len].1 = s;
                iced::Task::none()
            }
        }
    }
    pub fn check_command_running(&self) -> bool {
        self.task_chat_running || self.task_rag_running
    }
    pub fn check_arc_data(&self) -> iced::Task<Message> {
        if self.task_chat_running {
            let response_lock = self.response_lock.clone();
            iced::Task::perform(
                async move {
                    let r = response_lock.read().await;
                    r.to_string()
                },
                |s| OLLamaMessage::TaskSetContent(s).into(),
            )
        } else {
            iced::Task::none()
        }
    }
    pub fn draw(&self) -> Container<Message> {
        let mut col = Column::new();
        col = set_col_with_text_input(
            col,
            "ollama地址",
            |s| OLLamaMessage::InputBaseUrl(s),
            None,
            &self.base_url,
            "确定",
            &None::<String>,
        );
        col = set_col_with_text_input(
            col,
            "语言模型",
            |s| OLLamaMessage::InputLLModel(s),
            None,
            &self.l_l_model,
            "确定",
            &None::<String>,
        );
        col = if self.task_rag_running {
            set_col_with_text(
                col,
                "RAG模型",
                None::<Message>, 
                "生成中",
                "",
                &None::<String>)
        } else {
            set_col_with_text_input(
                col,
                "RAG模型",
                |s| OLLamaMessage::InputRAGModel(s),
                OLLamaMessage::ButtonGetRAGData.into(),
                &self.rag_model,
                "生成",
                &self.rag_building_err_msg,
            )
        };
        col = col.push(Container::new({
            let mut col = Column::new();
            for (role, content) in &self.contents {
                match role {
                    Roles::User => {
                        col = col.push(Container::new(
                            Row::new().push(Space::with_width(30)).push(
                                Text::new(content)
                                    .width(Length::Fill)
                                    .shaping(Shaping::Advanced)
                                    .align_x(iced::alignment::Horizontal::Right),
                            ),
                        ));
                    }
                    Roles::Assistant => {
                        col = col.push(Container::new(
                            Row::new()
                                .push(
                                    Text::new(content)
                                        .width(Length::Fill)
                                        .shaping(Shaping::Advanced)
                                        .align_x(iced::alignment::Horizontal::Left),
                                )
                                .push(Space::with_width(30)),
                        ));
                    }
                }
            }
            col
        }));
        col = set_col_with_text_input(
            col,
            "",
            |s| OLLamaMessage::InputAsk(s),
            if self.task_chat_running || self.rag_data.is_none() {
                None
            } else {
                Some(OLLamaMessage::ButtonAsk)
            },
            &self.ask_str,
            "确定",
            &self.ask_err_msg,
        );
        Container::new(col)
    }
}

#[derive(Debug,Clone)]
pub enum OLLamaMessage {
    InputBaseUrl(String),
    InputLLModel(String),
    InputRAGModel(String),
    ButtonGetRAGData,
    InputAsk(String),
    ButtonAsk,
    TaskDone(TaskResult),
    TaskSetContent(String),
}

#[derive(Clone)]
pub enum TaskResult {
    ChatTaskDone(Result<(),String>),
    RAGTaskDone(Result<InMemoryVectorStore<String>,String>)
}

impl std::fmt::Debug for TaskResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChatTaskDone(arg0) => f.debug_tuple("ChatTaskDone").field(arg0).finish(),
            Self::RAGTaskDone(_) => f.debug_tuple("RAGTaskDone").field(&String::new()).finish(),
        }
    }
}

impl Into<Message> for OLLamaMessage {
    fn into(self) -> Message {
        Message::AnalysisDatabaseMessage(AnalysisDatabaseMessage::OLLamaMessage(self))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OLLamaChatRequest {
    model: String,
    messages: Vec<OLLamaRequestMessage>,
    stream: bool,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OLLamaRequestMessage {
    role: String,
    content: String,
}

impl OLLamaChatRequest {
    fn new(model: impl ToString, messages: Vec<OLLamaRequestMessage>) -> Self {
        OLLamaChatRequest {
            model: model.to_string(),
            messages,
            stream: true,
            temperature: 0.5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OLLamaChatResponse {
    model: String,
    created_at: String,
    message: OLLamaRequestMessage,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OLLamaRAGRequest {
    model: String,
    input: Vec<String>,
}

impl OLLamaRAGRequest {
    fn new(model: impl ToString, input: Vec<String>) -> Self {
        OLLamaRAGRequest {
            model: model.to_string(),
            input,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct OLLamaRAGResponse {
    model: String,
    embeddings: Vec<Vec<f32>>,
    total_duration: i64,
    load_duration: i64,
    prompt_eval_count: i32,
}
