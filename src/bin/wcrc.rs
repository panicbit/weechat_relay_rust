#![allow(warnings)]
#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate weechat_relay;
extern crate tokio_core;
extern crate futures_await as futures;
extern crate error_chain;

use tokio_core::reactor::{Core,Handle};
use tokio_core::net::TcpStream;
use futures::prelude::*;
use error_chain::ChainedError;
use weechat_relay::{Client,Object,Result,ResultExt};

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    if let Err(e) = core.run(async_main(handle)) {
        println!("{}", e.display_chain());
    }
}

#[async]
fn async_main(handle: Handle) -> Result<()> {
    let pass = ::std::env::vars()
        .find(|&(ref k,_)| k == "WCP")
        .map(|(_,v)| v)
        .expect("Set the WCP env var to your relay password").clone();

    let addr = "192.168.2.11:3143".parse().unwrap();
    let conn = await!(TcpStream::connect(&addr, &handle))?;

    let client = await!(Client::auth(handle, conn, pass))?;

    println!("Ready.");

    let buffer = await!(client.infolist("buffer")).chain_err(|| "Error")?;

    for (i, item) in buffer.items().iter().enumerate() {
        if let Some(value) = item.get(&Object::str("full_name")) {
            println!("{}", value);
        }
        // println!("Buffer #{}:", i);
        // for (k,v) in item.0 {
        //     println!("{:.<25} = {}", k, v);
        // }
    }


    // loop {
    //     let stdin = ::std::io::stdin();
    //     let mut input = String::new();

    //     stdin.read_line(&mut input);

    //     let pong = await!(client.ping(input)).unwrap();

    //     println!("{:?}", pong);
    // }

    Ok(())
}
