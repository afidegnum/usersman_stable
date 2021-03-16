// // extern crate dotenv;
// use crate::config;
// use dotenv::dotenv;
// use std::env;
//
// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Message, SmtpTransport, Transport};
//
// use crate::auth::Confirmation;
// use crate::errors::ServiceError;
// //
// // // use crate::mail::{models::Confirmation, Confirmation}; //modify to add crate::members:: ...
// //
// // // TODO create a send mail, send bulk mail, fetch mail, a complete mail functionality api
// pub fn send_confirmation_mail(confirmation: &Confirmation) -> Result<(), ServiceError> {
//     dotenv().ok();
//     let configs = config::Config::from_env().unwrap();
//
//     // let domain_url = configs.domain_cnf.url;
//     // let expires = confirmation
//     //     .expires_at
//     //     .format("%I:%M %p %A, %-d %B, %C%y")
//     //     .to_string();
//     //
//     // let html_text = format!(
//     //     "Please click on the link below to complete registration. <br/>
//     //    <a href=\"{domain}/register/{id}\">Complete registration</a> <br/>
//     //   This link expires on <strong>{expires}</strong>",
//     //     domain = domain_url,
//     //     id = confirmation.id,
//     //     expires = expires
//     // );
//     // let plain_text = format!(
//     //     "Please visit the link below to complete registration:\n
//     //   {domain}/register/{id}\n
//     //   This link expires on {expires}.",
//     //     domain = domain_url,
//     //     id = confirmation.id,
//     //     expires = expires
//     // );
//
//     let email = Message::builder()
//         .from("NoBody <nobody@domain.tld>".parse().unwrap())
//         .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
//         .to("Hei <hei@domain.tld>".parse().unwrap())
//         .subject("Happy new year")
//         .body("Be happy!")
//         .unwrap();
//
//     let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());
//
//     // Open a remote connection to gmail
//     let mailer = SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();
//
//     // Send the email
//
//     let result = mailer.send(&email);
//
//     if result.is_ok() {
//         println!("Email sent");
//
//         Ok(())
//     } else {
//         println!("Could not send email: {:?}", result);
//
//         Err(ServiceError::ProcessError(String::from(
//             "Could not send confirmation email",
//         )))
//     }
// }

//////////////////////////
//
// use crate::config::Config;
// use crate::config::EndpointConfig;
//
// use crate::FormData;
// use ammonia::clean;
// use lettre::smtp::authentication::{Credentials, Mechanism};
// use lettre::smtp::extension::ClientId;
// use lettre::smtp::response::Response;
// use lettre::smtp::ConnectionReuseParameters;
// use lettre::transport::smtp::authentication::{Credentials, Mechanism};
// use lettre::transport::smtp::extension::ClientId;
// use lettre::transport::smtp::response::Response;
// use lettre::{SmtpClient, Transport};
// use lettre_email::Email;
// use v_htmlescape::escape;
//
// /// Send email via SMTP
// pub fn send_mail(
//     data: &FormData,
//     config: &Config,
//     endpoint_config: &EndpointConfig,
// ) -> Result<Response, lettre::smtp::error::Error> {
//     // Add this to the text if the user entered a phone number
//     let phone_contact = match data.phone.as_str() {
//         "" => "".to_string(),
//         _ => format!(" or call {}", data.phone),
//     };
//
//     // Construct Email
//     let email = Email::builder()
//         // Addresses can be specified by the tuple (email, alias)
//         .to((endpoint_config.target.email.clone(), endpoint_config.target.email_name.clone()))
//         // ... or by an address only
//         .from(endpoint_config.from_email.clone())
//         .reply_to(&data.email[..])
//         .subject(format!("[{endpoint_name}] Contact from {name}", endpoint_name=endpoint_config.name, name=data.name))
//         .alternative(
//             format!("<p>{name} wrote:</p><br><i>{message}</i>\n\n<br><br><p>Reply to <a href=\"mailto:{email}\">{email}</a> or {phone}</p>",
//                     name=escape(&clean(&data.name)),
//                     message=escape(&clean(&data.message)).to_string().replace("\n", "<br>"),
//                     phone=escape(&clean(&phone_contact)),
//                     email=escape(&clean(&data.email))),
//             format!("Message from {name}:\n\n> {message}\n\nReply to <{email}> or {phone}",
//                     name=escape(&clean(&data.name)),
//                     message=escape(&clean(&data.message)),
//                     phone=escape(&clean(&phone_contact)),
//                     email=escape(&clean(&data.email))))
//         .build()
//         .expect("Error while building mail");
//
//     // Connect to a SMTP-Server
//     let mut mailer = SmtpClient::new_simple(&config.server.smtp_server)
//         .unwrap()
//         // Set the name sent during EHLO/HELO, default is `localhost`
//         .hello_name(ClientId::Domain("atoav.com".to_string()))
//         // Add credentials for authentication
//         .credentials(Credentials::new(
//             config.server.smtp_user.to_string(),
//             config.server.smtp_password.to_string(),
//         ))
//         // Enable SMTPUTF8 if the server supports it
//         .smtp_utf8(true)
//         // Configure expected authentication mechanism
//         .authentication_mechanism(Mechanism::Plain)
//         // Enable connection reuse
//         .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
//         .transport();
//
//     mailer.send(email.into())
// }
