use chrono::{DateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ContactFormRequest {
    pub name: String,
    pub email: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[cfg_attr(feature = "csr", derive(Deserialize))]
pub struct ContactFormResponse {
    pub result: String,
}

#[derive(Debug, Clone)]
pub struct ContactSlot {
    pub date: NaiveDate,
    pub hour: u32,
    pub duration: u32,
}

// Construct slot from string
impl ContactSlot {
    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        use anyhow::Context;
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 3 {
            anyhow::bail!("Invalid slot format: {}", s);
        }
        let date = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d").context("Invalid date")?;
        let hour = parts[1].parse().context("Invalid hour")?;
        let duration = parts[2].parse().context("Invalid duration")?;
        Ok(Self { date, hour, duration})
    }
}

#[derive(Debug, Clone)]
pub struct ContactFormEntry {
    pub name: String,
    pub email: String,
    pub message: String,
    pub slot: Option<ContactSlot>,
    pub created_at: DateTime<Utc>,
}

// Construct entry from request
impl ContactFormEntry{
    pub fn new(req: &ContactFormRequest) -> anyhow::Result<Self> {
        if req.name.is_empty() {
            anyhow::bail!("Empty name");
        }
        if req.email.is_empty(){
            anyhow::bail!("Empty email");
        }
        if req.message.is_empty(){
            anyhow::bail!("Empty message");
        }
        let slot = match &req.slot {
            Some(s) => Some(ContactSlot::from_str(s)?),
            None => None,
        };
        Ok(Self {
            name: req.name.clone(),
            email: req.email.clone(),
            message: req.message.clone(),
            slot,
            created_at: Utc::now(),
        })
    }
}

// Prepare email body
impl ContactFormEntry {
    pub fn into_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        lines.push(format!("Name: {}", self.name));
        lines.push(format!("Email: {}", self.email));
        lines.push(format!("Message: {}", self.message));
        if let Some(slot) = &self.slot {
            lines.push(format!("Slot: {:?}", slot));
        }
        lines
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, clap::Args)]
pub struct EmailConfig {
    // Email destination
    #[clap(long, env = "EMAIL_TO")]
    pub to: String,

    // Sender user
    #[clap(long, env = "EMAIL_USER")]
    pub email_user: String,

    // Sender password
    #[clap(long, env = "EMAIL_PASSWORD", default_value = "")]
    pub email_password: String,

    // SMTP server host
    #[clap(long, env = "SMTP_SERVER", default_value = "localhost")]
    pub smtp_host: String,

    // SMTP server port
    #[clap(long, env = "SMTP_PORT", default_value = "1025")]
    pub smtp_port: u16,
}

// Constructor from OS vars
#[cfg(feature = "ssr")]
impl EmailConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            to: std::env::var("EMAIL_TO")?,
            email_user: std::env::var("EMAIL_USER")?,
            email_password: std::env::var("EMAIL_PASSWORD")?,
            smtp_host: std::env::var("SMTP_SERVER").unwrap_or("localhost".to_string()),
            smtp_port: std::env::var("SMTP_PORT")
                .unwrap_or("1025".to_string())
                .parse()
                .unwrap_or(1025)
        })
    }
}

// Send email implementation
#[cfg(feature = "ssr")]
impl ContactFormEntry {
    // Returns ID in the queue if successful
    pub fn notify(&self, email: &EmailConfig) -> anyhow::Result<String> {
        use lettre::message::{Mailbox, SinglePart};
        use lettre::Transport;
        use tracing::*;

        let mut mailbox_from: Mailbox = match email.email_user.parse() {
            Ok(m) => m,
            Err(e) => anyhow::bail!("Invalid email sender: {}", e.to_string()),
        };
        mailbox_from.name = Some("Robot".to_string());
        let mailbox_to: Mailbox = match email.to.parse() {
            Ok(m) => m,
            Err(e) => anyhow::bail!("Invalid email destination: {}", e.to_string()),
        };
        let creds = lettre::transport::smtp::authentication::Credentials::new(
            email.email_user.clone(),
            email.email_password.clone(),
        );
        let transport = lettre::SmtpTransport::builder_dangerous(&email.smtp_host)
            .credentials(creds)
            .port(email.smtp_port)
            .build();

        let body = SinglePart::builder()
            .header(lettre::message::header::ContentType::TEXT_PLAIN)
            .body(self.into_lines().join("\n"));
        let msg = lettre::Message::builder()
            .from(mailbox_from)
            .to(mailbox_to)
            .subject("Contact form submission")
            .singlepart(body)?;
        let out = transport.send(&msg)?;
        info!("Queued {:?}", out);
        Ok(out.message().take(1).collect::<Vec<_>>().join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot() {
        let slot = ContactSlot::from_str("2024-12-25|10|30").unwrap();
        assert_eq!(slot.date, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
        assert_eq!(slot.hour, 10);
        assert_eq!(slot.duration, 30);
    }

    #[test]
    #[ignore]
    fn test_notify(){
        std::env::set_var("EMAIL_TO", "to@test.com");
        std::env::set_var("EMAIL_USER", "user@test.com");
        std::env::set_var("EMAIL_PASSWORD", "password");
        std::env::set_var("SMTP_SERVER", "localhost");
        std::env::set_var("SMTP_PORT", "1025");

        let email = EmailConfig::from_env().unwrap();
        let rq = ContactFormRequest {
            name: "test".to_string(),
            email: "test@test.com".to_string(),
            message: "test".to_string(),
            slot: None,
        };
        let out = ContactFormEntry::new(&rq).unwrap().notify(&email).unwrap();
        println!("{}", out);
    }
}
