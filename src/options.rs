use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[arg(long)]
    pub smtp_server: String,
    #[arg(long)]
    pub smtp_username: String,
    #[arg(long)]
    pub smtp_password: String,
    #[arg(long)]
    pub from_email: String,
    #[arg(long)]
    pub to_email: String,
    #[arg(long)]
    pub email_subject: String,
    #[arg(long)]
    pub email_body: String,
    #[arg(long)]
    pub email_attachment: Option<String>,
}

// -s, --smtp-server         SMTP server for sending emails
// -p, --smtp-port           SMTP server port (Default: 587)
// -u, --smtp-username       Username for SMTP authentication
// -w, --smtp-password       Password for SMTP authentication
// -f, --from-email          Email address to send from
//
// -c, --config              Path to the SMTP json config file which replaces the above arguments
//
// -t, --to-email            Email addresses that will receive the email, can be multiples (comma-separated)
// -h, --subject             Subject of the email
// -b, --body                Body of the email
// -af, --attachments        File paths for attachments, can be multiples (comma-separated)
// -bf, --body-file          File path for email body
