// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadError> {
        let (response_sender, response_receiver) = std::sync::mpsc::sync_channel(1);
        let cmd = Command::Insert {
            draft: draft.clone(),
            response_channel: response_sender,
        };
        self.sender.try_send(cmd).map_err(|_| OverloadError)?;
        Ok(response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadError> {
        let (response_sender, response_receiver) =
            std::sync::mpsc::sync_channel::<Option<Ticket>>(1);
        let cmd = Command::Get {
            id: id,
            response_channel: response_sender,
        };

        self.sender.try_send(cmd).map_err(|_| OverloadError)?;
        Ok(response_receiver.recv().unwrap())
    }

    fn new(sender: SyncSender<Command>) -> TicketStoreClient {
        TicketStoreClient { sender: sender }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient::new(sender)
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaderd")]
pub struct OverloadError;

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
