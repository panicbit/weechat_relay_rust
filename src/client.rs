use futures::prelude::*;
use error_chain::ChainedError;
use errors::*;
use tokio_core::reactor::Handle;
use tokio_io::{AsyncRead,AsyncWrite};
use tokio_io::io::{ReadHalf};
use raw::Compression;
use raw;
use command::{self,Command};
use message::{self,Message};
use message_resolver::{MessageResolver,Resolver};
use object;
use futures::sync::mpsc::{unbounded,UnboundedSender,UnboundedReceiver};
use futures::sync::oneshot::{channel,Sender,Receiver};
use typemap::{TypeMap,Key};
use std::rc::Rc;
use std::cell::{Cell,RefCell};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::collections::VecDeque;

pub struct Client {
    id_counter: Rc<Cell<usize>>,
    command_tx: UnboundedSender<Command>,
    message_resolver: Rc<RefCell<MessageResolver>>,
}

impl Client {
    #[async]
    pub fn auth<S,P>(handle: Handle, stream: S, pass: P) -> Result<Self> where
        S: AsyncRead + AsyncWrite + 'static,
        P: Into<String> + 'static,
    {
        let (command_tx, command_rx) = unbounded();
        
        let client = Client {
            id_counter: Rc::new(Cell::new(0)),
            command_tx,
            message_resolver: Rc::new(RefCell::new(MessageResolver::new())),
        };

        let (reader, writer) = stream.split();
        let writer = await!(raw::send_init(writer, pass.into(), Compression::Off))?;
        let command_sender = command_sender(writer, command_rx);
        let message_receiver = message_receiver(reader, client.message_resolver.clone());
        let tasks = command_sender
            .select(message_receiver)
            .map(|_| ())
            .map_err(|_| ());

        handle.spawn(tasks);

        await!(client.ping("auth")).chain_err(|| ErrorKind::AuthFailed)?;

        println!("Auth success!");

        Ok(client)
    }

    fn new_id(&self) -> String{
        let id = self.id_counter.get();
        self.id_counter.set(id + 1);
        id.to_string()
    }

    pub fn ping<S: Into<String>>(&self, msg: S) -> Receiver<String> {
        self.command_tx.unbounded_send(command::Ping(msg.into()).into());
        self.message_resolver.borrow_mut().register_pong()
    }

    pub fn infolist<S: Into<String>>(&self, name: S) -> Receiver<object::InfoList> {
        let id = self.new_id();
        self.command_tx.unbounded_send(command::InfoList(id.clone(), name.into()).into());
        self.message_resolver.borrow_mut().register_promise(id)
    }
}

#[async]
fn command_sender<W>(mut writer: W, command_rx: UnboundedReceiver<Command>) -> Result<()> where
    W: AsyncWrite + 'static,
{
    #[async]
    for command in command_rx.map_err(|_| ErrorKind::Disconnected) {
        writer = await!(command.send_raw(writer))?;
    }

    println!("Quitting sender :(");

    Ok(())
}

#[async]
fn message_receiver<R>(mut reader: R, message_resolver: Rc<RefCell<MessageResolver>>) -> Result<()> where
    R: AsyncRead + 'static,
{
    loop {
        let (mut r, message) = match await!(raw::read_message(reader)) {
            Ok(res) => res,
            Err(err) => {
                println!(">>> {:?}", err);
                break
            }
        };

        if let Err(err) = message_resolver.borrow_mut().resolve(&mut message.as_slice()).chain_err(|| "resolving message") {
            println!(">>> {}", err.display_chain());
        };

        reader = r;
    }

    // TODO: Allow canceling of existing promises

    println!("Quitting receiver :(");
    bail!(ErrorKind::Disconnected)
}
