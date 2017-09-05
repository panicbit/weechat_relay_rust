use typemap::{TypeMap,Key};
use std::collections::{HashMap,VecDeque};
use std::io::Read;
use std::convert::TryFrom;
use std::marker::PhantomData;
use futures::sync::oneshot;
use message::{self,Id};
use object::{self,DecodableObject};
use errors::*;

pub(crate) struct MessageResolver {
    resolvers: HashMap<String, Box<Resolver>>,
    pong_promises: VecDeque<oneshot::Sender<String>>,
}

impl MessageResolver {
    pub fn new() -> Self {
        MessageResolver {
            resolvers: HashMap::new(),
            pong_promises: VecDeque::new(),
        }
    }

    pub fn resolve<R: Read>(&mut self, r: &mut R) -> Result<()> {
        let id = Id::try_from(r)
            .chain_err(|| "decoding id")?;

        match id {
            Id::Pong => {
                self.pong_promises
                    .pop_front().ok_or(ErrorKind::MissingResponsePromise)?
                    .send(message::Pong::try_from(r).chain_err(|| "decoding pong")?.0.into());
            },
            Id::Other(id) => {
                self.resolvers
                    .remove(&id).ok_or(ErrorKind::MissingResponsePromise)?
                    .resolve(r).chain_err(|| "using resolver")?;
            },
            _ => unimplemented!(),
        }

        Ok(())
    }

    pub fn register<T: Resolver + 'static>(&mut self, id: String, resolvable: T) {
        self.resolvers.insert(id, Box::new(resolvable));
    }

    pub fn register_promise<T>(&mut self, id: String) -> oneshot::Receiver<T> where
        oneshot::Sender<T>: Resolver,
        T: 'static,
    {
        let (tx, rx) = oneshot::channel();
        self.register(id, tx);
        rx
    }

    pub fn register_pong(&mut self) -> oneshot::Receiver<String> {
        let (tx, rx) = oneshot::channel();
        self.pong_promises.push_back(tx);
        rx
    }
}

pub(crate) trait Resolver {
    fn resolve(self: Box<Self>, r: &mut Read) -> Result<()>;
}

impl Resolver for oneshot::Sender<object::InfoList> {
    fn resolve(self: Box<Self>, r: &mut Read) -> Result<()> {
        let infolist = object::InfoList::decode(r).chain_err(|| "decoding InfoList")?;
        self.send(infolist);
        Ok(())
    }
}
