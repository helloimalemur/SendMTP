use std::thread;
use std::time::Duration;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::{ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::client::TlsVersion::Tlsv13;

pub fn send_mail(smtp_from: String, smtp_host: String, subject: String, message: String, recipients: Vec<String>, username: String, password: String, email_attachment: Option<String>) {

    recipients.iter().for_each(|recipient| {
        let email = Message::builder()
            .from(smtp_from.parse().unwrap())
            .reply_to(smtp_from.parse().unwrap())
            .to(recipient.parse().unwrap())
            .subject(subject.clone())
            .header(ContentType::TEXT_PLAIN)
            .body(message.clone())

            .unwrap();

        let creds = Credentials::new(username.to_owned(), password.to_owned());

        let mailer = SmtpTransport::relay(smtp_host.as_str())
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Err(e) => println!("Could not send email: {e:?}"),
            Ok(_) => (),
        }
        thread::sleep(Duration::new(1, 500000000)); // sleep for 1.5 seconds
    });
}
