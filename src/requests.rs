use super::components::{LcnCommand, Schedule};
use super::systems::status_reporter::TaskStatus;
use lame_ecs::Entity;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;

pub enum Request {
    NewTask((mpsc::SyncSender<Response>, TaskRequest)),
    RemoveTask((mpsc::SyncSender<Response>, Entity)),
    GetStatus(mpsc::SyncSender<Response>),
}

pub enum Response {
    NewTask(Entity),
    RemoveTask(bool),
    GetStatus(Vec<TaskStatus>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskRequest {
    pub schedule: Schedule,
    pub cmd: LcnCommand,
}

pub fn make_request(
    tx: &mpsc::SyncSender<Request>,
    rx: mpsc::Receiver<Response>,
    request: Request,
) -> Result<Response, RequestError> {
    tx.try_send(request)?;
    Ok(rx.recv()?)
}

#[derive(Debug)]
pub enum RequestError {
    Send(mpsc::TrySendError<Request>),
    Recv(mpsc::RecvError),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Send(e) => e.fmt(f),
            RequestError::Recv(e) => e.fmt(f),
        }
    }
}

impl From<mpsc::TrySendError<Request>> for RequestError {
    fn from(e: mpsc::TrySendError<Request>) -> Self {
        RequestError::Send(e)
    }
}

impl From<mpsc::RecvError> for RequestError {
    fn from(e: mpsc::RecvError) -> Self {
        RequestError::Recv(e)
    }
}
