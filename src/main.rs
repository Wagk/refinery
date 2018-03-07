extern crate futures;
extern crate hyper;
extern crate tokio_core;

// use std::io::{self, Write};
// use futures::{Future, Stream, Poll, Async};
// use futures::executor::spawn;
// use hyper::{Client};
// use tokio_core::reactor::Core;

mod query;

fn main()
{
    let url = "http://api.xivdb.com/item".to_string();
    let text = query::xivdb(&url);

    println!("Printout");
    println!("{:?}", text.raw);

    // let mut core = Core::new().unwrap();

    // let handle = core.handle();
    // let client = Client::new(&handle);

    // // so I can override the parse trait to use the hyper::Uri impl instead
    // // the ::<> bit is known as the "turbofish" syntax (?!)
    // let url = "http://api.xivdb.com/item".parse::<hyper::Uri>().unwrap();

    // let work = client.get(url).and_then(|res|{
    //     println!("Response: {}", res.status());
    //     println!("Headers: {}", res.headers());

    //     res.body().for_each(|chunk|{
    //         io::stdout().write_all(&chunk).map_err(From::from)
    //     })
    // });

    // spawn(work).wait_future().unwrap();

    // // core.run(work).unwrap();
}
