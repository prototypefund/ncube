use async_trait::async_trait;
use std::fmt::Debug;
use tokio::sync::mpsc::{self, Sender};
use xactor::{message, Actor, Context, Handler};

use crate::actors::Registry;
use crate::errors::{ActorError, HostError};
use crate::tasks::create_workspace;

#[derive(Debug)]
enum TaskMessage {
    SetupWorkspace(String),
}

pub(crate) struct TaskActor {
    tx: Sender<TaskMessage>,
}

impl Actor for TaskActor {}

impl Registry for TaskActor {}

impl TaskActor {
    pub fn new() -> Result<Self, HostError> {
        let (tx, mut rx) = mpsc::channel(100);

        tokio::spawn(async move {
            while let Some(res) = rx.recv().await {
                match res {
                    TaskMessage::SetupWorkspace(location) => create_workspace(location)
                        .await
                        .expect("Failed to create workspace"),
                }
            }
        });

        Ok(TaskActor { tx })
    }
}

#[message(result = "Result<(), ActorError>")]
#[derive(Debug)]
pub(crate) struct SetupWorkspace {
    pub(crate) location: String,
}

#[async_trait]
impl Handler<SetupWorkspace> for TaskActor {
    async fn handle(
        &mut self,
        _ctx: &Context<Self>,
        msg: SetupWorkspace,
    ) -> Result<(), ActorError> {
        self.tx
            .send(TaskMessage::SetupWorkspace(msg.location))
            .await?;
        Ok(())
    }
}
