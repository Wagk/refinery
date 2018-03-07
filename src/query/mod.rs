// This module is responsible for asking xivdb things
extern crate reqwest;

pub struct Data {
    pub raw: String
}

pub fn xivdb(endpoint: &String) -> Data
{
    let client = reqwest::Client::new();

    println!("Printout");
    // TODO: I don't get why this requires a string reference and not a string
    // let mut response = client.get(endpoint).send().unwrap();
    // let mut response = client.get("http://api.xivdb.com/item").send().unwrap();
    let mut response = client.get(endpoint).send();
    match response {
        Ok(response.unwrap()) => println!("Send was successful"),
        Err(err) => println!("{:?}", err)
    }

    println!("Printout");
    // NOTE: prints OK with hardcoded string
    // println!("{:?}", response.status());

    let string = response.json().unwrap();

    Data{raw: string}
}
