extern crate imap;
use native_tls::TlsConnector;
use mail_parser::*;
use std::env;

struct Config{
    email: String,
    password: String,
}

impl Config{
    fn new(args: &Vec<String>)->Self{
        Config { email: args[1].clone(), password: args[2].clone() }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    fetch_inbox_top(&config);
}



fn fetch_inbox_top(config: &Config)->imap::error::Result<Option<String>>{
    let domain = "imap.gmail.com";
    let port: u16 = 993;
    let tls = TlsConnector::builder().build().unwrap();

    let client = imap::connect((domain, port), domain, &tls).unwrap();

    let mut imap_session = client.login(&config.email,&config.password).map_err(|e| e.0)?;

    imap_session.select("INBOX")?;

    let messages = imap_session.fetch("1", "RFC822")?;
    //println!("{:#?}", messages);
    let message = if let Some(m) = messages.iter().next() {
        //println!("{:?}", m);
        m
    } else {
        return Ok(None);
    };

    let body1 = message.body().unwrap();
    let body = std::str::from_utf8(body1)
        .expect("message was not valid utf-8")
        .to_string();
    //println!("Message body: {}", body);

    let parsed = Message::parse(body1).unwrap();

    println!("{:#?}", parsed.subject().unwrap());

    let mut subjects = Vec::new();

    let headers = imap_session.uid_fetch("1", "RFC822.HEADER");

    for msg in headers.iter(){
        let msg = msg.first().unwrap();
        subjects.push(msg);
    }

    let mut utf_vec = Vec::new();
    for s in subjects.iter(){
        utf_vec.push(std::str::from_utf8(s.header().unwrap()))
    }

    //println!("{:#?}", utf_vec);

    imap_session.logout();

    Ok(Some(body))
}
