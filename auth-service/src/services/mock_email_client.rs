use crate::domain::{email::Email, email_client::EmailClient};

#[derive(Clone)]
pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email(&self, recepient: &Email, subject: &str, content: &str) -> Result<(), String> {
        println!( "sending email to {} with subject: {} and content: {}", recepient.as_ref(), subject, content);

        Ok(())
     }
}
