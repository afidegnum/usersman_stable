use crate::config;
use crate::errors::ServiceError;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Address, Message, SmtpTransport, Transport};

#[derive(Debug)]
pub struct SendMail {
    pub from_name: String,
    pub from: String,
    pub reply_name: String,
    pub reply_to: String,
    pub to_name: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

// TODO CUSTOMIZE Name for each recipient
impl SendMail {
    pub async fn send(&self) -> Result<Response, ServiceError> {
        let configs = config::Config::from_env().unwrap();

        let from = "user@localhost".to_string();
        let details = from.split("@").collect::<Vec<&str>>();
        let address_from = Address::new(details[0], details[1]).unwrap();
        let from_name = "My Name".into();
        let fromx = Mailbox::new(Some(from_name), address_from);

        let replx = fromx.clone();

        let to = "user@localhost".to_string();
        let details_to = from.split("@").collect::<Vec<&str>>();
        let address_to = Address::new(details_to[0], details_to[1]).unwrap();
        let to_name = "Recipient Name".into();
        let sendx = Mailbox::new(Some(to_name), address_to);

        let email = Message::builder()
            .from(fromx)
            .to(sendx)
            .reply_to(replx)
            .subject(&self.subject)
            .body(String::from(&self.body))
            .unwrap();

        let creds = Credentials::new(
            configs.mail_cnf.username.to_string(),
            configs.mail_cnf.password.to_string(),
        );

        // TODO uncomment below for remote server connection
        // let mailer = SmtpTransport::relay("localhost").unwrap().port(1025).credentials(creds).build();
        // let mailer = SmtpTransport::relay(&configs.mail_cnf.host.to_string())
        //
        //     .unwrap()
        //     .credentials(creds)
        //     .build();

        let mailer = SmtpTransport::builder_dangerous("localhost")
            .port(1025)
            .credentials(creds)
            .build();
        // println!("{:#?}", &email);

        // Send the email
        let response = mailer.send(&email).map_err(ServiceError::Smtp)?;

        Ok(response)
    }
}
