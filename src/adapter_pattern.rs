use std::fmt::format;


pub trait PaymentGateway {
    fn charge(&self, amount_paise: u64, description: &str) -> Result<String, String>;
    fn refund(&self, transaction_id:&str)-> Result<(),String>;
    fn name(&self) -> &str;
}


//adapter 1
struct StripeClient {
    api_key: String,
}

impl StripeClient {
    fn new(api_key: &str) -> Self{
        Self { api_key: api_key.to_string() }
    }

    fn create_charge(&self, amount_cents: u64, desc: &str) -> Result<String,String>{
        println!(
            " [Stripe] Post /charge amount={} desc= '{}' key={} ...",amount_cents,desc,&self.api_key[..8]
        );
        Ok(format!("ch_spripe_{}", amount_cents))
    }

    fn issue_refund(&self, changre_id: &str) -> Result<(), String> {
        println!(" [Strioe] POST / refunds charge_id={}", changre_id);
        Ok(())
    }
}


//adapter 2
struct RazorpaySDK {
    key_id:    String,
    key_secret: String,
}


impl RazorpaySDK {
    fn new(key_id: &str, key_secret: &str) -> Self {
        Self {
            key_id:     key_id.to_string(),
            key_secret: key_secret.to_string(),
        }
    }

    fn initiate_payment(&self, paise: u64, notes: &str) -> Result<String, String> {
        println!(
            "  [Razorpay] POST /orders amount={} notes='{}' key={}...",
            paise, notes, &self.key_id[..6]
        );
        Ok(format!("order_razorpay_{}", paise))
    }

    fn cancel_payment(&self, order_id: &str) -> Result<(), String> {
        println!("  [Razorpay] POST /payments/{}/refund", order_id);
        Ok(())
    }
}

//adapter 3
struct PayPalClient {
    client_id: String,
    sandbox:   bool,
}

impl PayPalClient {
    fn new(client_id: &str, sandbox: bool) -> Self {
        Self { client_id: client_id.to_string(), sandbox }
    }

    // PayPal takes amount in USD as f64
    fn execute_transaction(&self, usd_amount: f64, memo: &str) -> Result<String, String> {
        let env = if self.sandbox { "sandbox" } else { "live" };
        println!(
            "  [PayPal][{}] POST /payments amount=${:.2} memo='{}'",
            env, usd_amount, memo
        );
        Ok(format!("PAY-paypal-{:.0}", usd_amount * 100.0))
    }

    fn void_transaction(&self, payment_id: &str) -> Result<(), String> {
        println!("  [PayPal] POST /payments/{}/void", payment_id);
        Ok(())
    }
}

//adapter 1
pub struct StripeAdapter {
    client: StripeClient,
}

impl StripeAdapter {
   pub  fn new(api_key: &str) -> Self {
        Self {
            client: StripeClient::new(api_key)
        }
    }
}


impl PaymentGateway for StripeAdapter {
    fn charge(&self, amount_paise: u64, description: &str) -> Result<String, String> {
        let cents = amount_paise/83;
        self.client.create_charge(cents, description)
    }

    fn refund(&self, transaction_id: &str) -> Result<(), String> {
        self.client.issue_refund(transaction_id)
    }

    fn name(&self) -> &str {
        "Stripe"
    }
}

//adapter 2


pub struct RazorpayAdapter {
    sdk: RazorpaySDK,
}

impl RazorpayAdapter {
   pub fn new(key_id: &str, key_secret: &str) -> Self {
        Self { sdk: RazorpaySDK::new(key_id, key_secret) }
    }
}

impl PaymentGateway for RazorpayAdapter {
    fn charge(&self, amount_paise: u64, description: &str) -> Result<String, String> {
        self.sdk.initiate_payment(amount_paise, description)
    }

    fn refund(&self, transaction_id: &str) -> Result<(), String> {
        self.sdk.cancel_payment(transaction_id)
    }

    fn name(&self) -> &str { "Razorpay" }
}

//adapter 3
pub struct PayPalAdapter {
    client: PayPalClient,
}

impl PayPalAdapter {
    pub fn new(client_id: &str, sandbox: bool) -> Self {
        Self { client: PayPalClient::new(client_id, sandbox) }
    }
}

impl PaymentGateway for PayPalAdapter {
    fn charge(&self, amount_paise: u64, description: &str) -> Result<String, String> {
        
        let usd = amount_paise as f64 / 100.0 / 83.0;
        self.client.execute_transaction(usd, description)
    }

    fn refund(&self, transaction_id: &str) -> Result<(), String> {
        self.client.void_transaction(transaction_id)
    }

    fn name(&self) -> &str { "PayPal" }
}


//Order service know nothing about Strioe , razorpay or paypal only depends on paymentgateway trait

pub struct OrderService {
    gateway: Box<dyn PaymentGateway>,
}

impl OrderService {
    pub fn new(gateway: Box<dyn PaymentGateway>) -> Self {
        Self { gateway }
    }

   pub fn checkout(&self, order_id: u32, amount_paise: u64) -> Result<String, String> {
        println!(
            "\nProcessing order #{} via {} for {} paise",
            order_id, self.gateway.name(), amount_paise
        );

        let txn_id = self.gateway
            .charge(amount_paise, &format!("Order #{}", order_id))?;

        println!("  Transaction ID: {}", txn_id);
        Ok(txn_id)
    }

   pub fn cancel_order(&self, txn_id: &str) {
        println!("\nCancelling transaction {}", txn_id);
        match self.gateway.refund(txn_id) {
            Ok(_)  => println!("  Refund successful"),
            Err(e) => println!("  Refund failed: {}", e),
        }
    }
}
 