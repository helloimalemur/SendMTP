use clap::Parser;
use crate::email::send_mail;
use crate::options::Arguments;

mod options;
mod email;

fn main() {

    let arguments = Arguments::parse();
    println!("{:?}", arguments);

    let smtp_server = arguments.smtp_server;
    let smtp_port = arguments.smtp_port;
    let smtp_username = arguments.smtp_username;
    let smtp_password = arguments.smtp_password;
    let from_email = arguments.from_email;
    let to_email = arguments.to_email;
    let email_subject = arguments.email_subject;
    let email_body = arguments.email_body;
    let email_attachment = arguments.email_attachment;



    let recipients = vec![to_email];
    send_mail(from_email, smtp_server, smtp_port, email_subject, email_body, recipients, smtp_username, smtp_password, email_attachment);
}
