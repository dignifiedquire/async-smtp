// this example can be used as a reference for other smtp providers
// tested with gmail, outlook smtp servers

use async_native_tls::TlsConnector;
use async_smtp::authentication::{Credentials, Mechanism};
use async_smtp::{Envelope, SendableEmail, SmtpClient, SmtpTransport};
use tokio::io::BufStream;
use tokio::net::TcpStream;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

// https://developers.google.com/gmail/imap/imap-smtp
const SMTP_SERVER: &str = "smtp.gmail.com";
const TLS_PORT: u16 = 587;

const USER: &str = "user@gmail.com";
const ACCESS_TOKEN: &str = "accesstoken";

#[tokio::main]
async fn main() -> Result<()> {
    let stream = BufStream::new(TcpStream::connect((SMTP_SERVER, TLS_PORT)).await?);

    let client = SmtpClient::new();
    let transport = SmtpTransport::new(client, stream).await?;

    let pre_tls_stream = transport.starttls().await?.into_inner();
    let tls_stream = TlsConnector::new()
        .connect(SMTP_SERVER, pre_tls_stream)
        .await?;

    let client = SmtpClient::new().without_greeting(); // do not wait an greeting message after STARTTLS
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
