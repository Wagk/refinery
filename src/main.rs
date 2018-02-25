extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client};
use tokio_core::reactor::Core;

fn main()
{
    let mut core = Core::new().unwrap();

    let handle = core.handle();
    let client = Client::new(&handle);

    // so I can override the parse trait to use the hyper::Uri impl instead
    // the ::<> bit is known as the "turbofish" syntax (?!)
    let url = "http://api.xivdb.com/item".parse::<hyper::Uri>().unwrap();

    let work = client.get(url).and_then(|res|{
        println!("Response: {}", res.status());
        println!("Headers: {}", res.headers());

        res.body().for_each(|chunk|{
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_|{
        println!("\n\nDone");
    });

    // core.run(work).unwrap();
}
