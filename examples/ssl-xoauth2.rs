// this example can be used as a reference for other smtp providers
// tested with gmail smtp server

use async_native_tls::TlsConnector;
use async_smtp::authentication::{Credentials, Mechanism};
use async_smtp::{Envelope, SendableEmail, SmtpClient, SmtpTransport};
use tokio::io::BufStream;
use tokio::net::TcpStream;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

// https://developers.google.com/gmail/imap/imap-smtp
const SMTP_SERVER: &str = "smtp.gmail.com";
const SSL_PORT: u16 = 465;

const USER: &str = "user@gmail.com";
const ACCESS_TOKEN: &str = "accesstoken";

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect((SMTP_SERVER, SSL_PORT)).await?;
    let tls_stream = TlsConnector::new().connect(SMTP_SERVER, stream).await?;

    let client = SmtpClient::new();
    let mut transport = SmtpTransport::new(client, BufStream::new(tls_stream)).await?;

    let credentials = Credentials::new(USER.to_owned(), ACCESS_TOKEN.to_owned());
    transport
        .auth(Mechanism::Xoauth2, &credentials) // use modern auth mechanism
        .await?;

    let email = SendableEmail::new(
        Envelope::new(
            Some(USER.parse().unwrap()),
            vec!["root@localhost".parse().unwrap()],
        )?,
        "Hello world",
    );
    transport.send(email).await?;

    Ok(())
}
