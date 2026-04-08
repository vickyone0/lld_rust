

//implementation axis
pub trait NotificationSender {
    fn send(&self, recipient: &str, subject: &str, body: &str);
    fn channel_name(&self) -> &str;
}

// Concrete impl 1: Email
pub struct EmailSender {
    smtp_host: String,
}

impl EmailSender {
    pub fn new(host: &str) -> Self {
        Self { smtp_host: host.to_string() }
    }
}

impl NotificationSender for EmailSender {
    fn send(&self, recipient: &str, subject: &str, body: &str) {
        println!(
            "  [Email via {}] To: {} | Subject: {} | Body: {}",
            self.smtp_host, recipient, subject, body
        );
    }
    fn channel_name(&self) -> &str { "Email" }
}


// Concrete impl 2: SMS
pub struct SmsSender {
    api_key: String,
}

impl SmsSender {
   pub fn new(api_key: &str) -> Self {
        Self { api_key: api_key.to_string() }
    }
}

impl NotificationSender for SmsSender {
    fn send(&self, recipient: &str, _subject: &str, body: &str) {
        let truncated = if body.len() > 60 {
            format!("{}...", &body[..60])
        } else {
            body.to_string()
        };
        println!(
            "  [SMS via Twilio key={}...] To: {} | {}",
            &self.api_key[..6], recipient, truncated
        );
    }
    fn channel_name(&self) -> &str { "SMS" }
}


// Concrete impl 3: Push notification
pub struct PushSender {
    fcm_project: String,
}

impl PushSender {
    pub fn new(project: &str) -> Self {
        Self { fcm_project: project.to_string() }
    }
}

impl NotificationSender for PushSender {
    fn send(&self, recipient: &str, subject: &str, body: &str) {
        println!(
            "  [Push via FCM project={}] Device: {} | {}: {}",
            self.fcm_project, recipient, subject, body
        );
    }
    fn channel_name(&self) -> &str { "Push" }
}




//Abstraction axis

pub trait Notification {
    fn notify(&self, recipient: &str, message: &str);
}

// Abstraction 1: Low urgency — plain message, no frills
pub struct LowUrgencyNotification {
    sender: Box<dyn NotificationSender>,   
}

impl LowUrgencyNotification {
    pub fn new(sender: Box<dyn NotificationSender>) -> Self {
        Self { sender }
    }
}

impl Notification for LowUrgencyNotification {
    fn notify(&self, recipient: &str, message: &str) {
        self.sender.send(
            recipient,
            "FYI",
            message,
        );
    }
}

// Abstraction 2: High urgency — prefixes subject, adds context
pub struct HighUrgencyNotification {
    sender: Box<dyn NotificationSender>,
    system: String,
}

impl HighUrgencyNotification {
    pub fn new(sender: Box<dyn NotificationSender>, system: &str) -> Self {
        Self { sender, system: system.to_string() }
    }
}

impl Notification for HighUrgencyNotification {
    fn notify(&self, recipient: &str, message: &str) {
        self.sender.send(
            recipient,
            &format!("[URGENT] {} Alert", self.system),
            &format!("Immediate action required: {}", message),
        );
    }
}

// Abstraction 3: Critical — sends on ALL channels via a list
pub struct CriticalNotification {
    senders: Vec<Box<dyn NotificationSender>>,  
}

impl CriticalNotification {
   pub fn new(senders: Vec<Box<dyn NotificationSender>>) -> Self {
        Self { senders }
    }
}

impl Notification for CriticalNotification {
    fn notify(&self, recipient: &str, message: &str) {
        println!("  *** CRITICAL — broadcasting on {} channels ***",
            self.senders.len());
        for sender in &self.senders {
            sender.send(
                recipient,
                "CRITICAL ALERT",
                &format!("!!! {} !!!", message),
            );
        }
    }
}


//alert service use only notification abstraction

pub struct AlertService;

impl AlertService {
    pub fn send_alert(&self, notif: &dyn Notification, recipient: &str, msg: &str) {
        notif.notify(recipient, msg);
    }
}