use futures::prelude::*;
use tokio_io::io;
use tokio_io::AsyncWrite;
use Result;

pub trait IdGen {
    fn gen_id(&mut self) -> String;
}

#[derive(Clone,Debug)]
pub enum Command {
    Ping(Ping),
    InfoList(InfoList),
}

impl Command {
    #[async]
    pub(crate) fn send_raw<W: AsyncWrite + 'static>(self, writer: W) -> Result<W> {
        let writer = match self {
            Command::Ping(c)     => await!(c.send_raw(writer))?,
            Command::InfoList(c) => await!(c.send_raw(writer))?,
        };

        Ok(writer)
    }
}

#[derive(Clone,Debug)]
pub struct Ping(pub String);

impl Ping {
    #[async]
    pub(crate) fn send_raw<W: AsyncWrite + 'static>(self, writer: W) -> Result<W> {
        let Ping(msg) = self;
        let data = format!("ping {}\n", msg);
        let (writer, _) = await!(io::write_all(writer, data))?;

        Ok(writer)
    }
}

impl From<Ping> for Command {
    fn from(c: Ping) -> Self {
        Command::Ping(c)
    }
}

#[derive(Clone,Debug)]
pub struct InfoList(pub String, pub String);

impl InfoList {
    #[async]
    pub(crate) fn send_raw<W: AsyncWrite + 'static>(self, writer: W) -> Result<W> {
        let InfoList(id, name) = self;
        let data = format!("({}) infolist {}\n", id, name);
        let (writer, _) = await!(io::write_all(writer, data))?;

        Ok(writer)
    }
}

impl From<InfoList> for Command {
    fn from(c: InfoList) -> Self {
        Command::InfoList(c)
    }
}
