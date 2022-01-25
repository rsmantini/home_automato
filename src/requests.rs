use super::components::{LcnCommand, Schedule};
use super::systems::status_reporter::TaskStatus;
use lame_ecs::Entity;
use rocket::tokio::sync::{mpsc, oneshot};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Request {
    NewTask((oneshot::Sender<Response>, TaskRequest)),
    RemoveTask((oneshot::Sender<Response>, Entity)),
    GetStatus(oneshot::Sender<Response>),
}

#[derive(Debug)]
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
    tx: &mpsc::UnboundedSender<Request>,
    mut rx: oneshot::Receiver<Response>,
    request: Request,
) -> Result<Response, RequestError> {
    tx.send(request)?;
    let mut result = Err(oneshot::error::TryRecvError::Empty);
    while let Err(oneshot::error::TryRecvError::Empty) = result {
        result = rx.try_recv();
    }
    result.map_err(RequestError::from)
}

#[derive(Debug)]
pub enum RequestError {
    Send(mpsc::error::SendError<Request>),
    Recv(oneshot::error::TryRecvError),
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Send(e) => e.fmt(f),
            RequestError::Recv(e) => e.fmt(f),
        }
    }
}

impl From<mpsc::error::SendError<Request>> for RequestError {
    fn from(e: mpsc::error::SendError<Request>) -> Self {
        RequestError::Send(e)
    }
}

impl From<oneshot::error::TryRecvError> for RequestError {
    fn from(e: oneshot::error::TryRecvError) -> Self {
        RequestError::Recv(e)
    }
}
