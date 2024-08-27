use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use std::thread;
use std::time::Duration;

pub async fn send_mail(
    smtp_from: String,
    smtp_host: String,
    subject: String,
    message: String,
    recipients: Vec<String>,
    username: String,
    password: String,
    email_attachment: Option<String>,
) {
    let message = MessageBuilder::new()
        .from((smtp_from))
        .to(recipients)
        .subject(subject)
        .html_body(message.clone())
        .text_body(message.clone());

    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    SmtpClientBuilder::new(smtp_host, 465)
        .implicit_tls(true)
        .credentials((username, password))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
