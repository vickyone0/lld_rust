mod defult_method;
mod dyn_trait;
mod trait_object;
mod stratergy_pattern;
mod adapter_pattern;
mod bridge_pattern;
mod chainofresponsibility_pattern;
mod command_pattern;

//use defult_method::*;
use defult_method::Describe;
//use dyn_trait::*;
use trait_object::*;
use stratergy_pattern::*;
use adapter_pattern::*;
use bridge_pattern::*;
use chainofresponsibility_pattern::*;
use command_pattern::*;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    // let dog = Dog { name: String::from("Buddy") };
    // let robot = Robot { id: 1 };

    // println!("{}", dog.shout());
    // println!("{}", robot.shout());
    // println!("{}", robot.describe());
    // println!("{}", dog.describe());

    // let animals: Vec<Box<dyn Speak>> = vec![
    //     Box::new(dyn_trait::Dog { name: String::from("Buddy") }),
    //     Box::new(dyn_trait::Cat { name: String::from("Whiskers") }),
    //     Box::new(dyn_trait::Robot { id: 1 }),
    // ];

    // for animal in animals {
    //     println!("{}", animal.introduce());
    // }

    // let dog = Dog { name: String::from("Buddy") };
    // let cat = Cat { name: String::from("Whiskers") };
    // let dog2 = Dog { name: String::from("Rex") };

    // announce(cat.clone());
    // loudest(dog.clone(), dog2.clone());
    // debug_announce(dog);
    // pair_up(cat, dog2);

    // let mut calculator = PriceCalculator::new(Box::new(RegularPrice));

    // println!("--- Regular Price ---");
    // calculator.calculate(100.0);

    // println!("\n--Swap to Discount Price ---");
    // calculator.set_strategy(Box::new(DiscountPrice { discount_percent: 20.0 }));
    // // calculator.calculate(100.0);

    // println!("\n--Swap to Seasonal Price ---");
    // calculator.set_strategy(Box::new(SeasonalPrice { multiplier: 1.2 }));
    // calculator.calculate(100.0);

    // println!("\n--Swap to Bulk Price ---");
    // calculator.set_strategy(Box::new(BulkPrice {
    //     quantity: 10,
    //     tier_threshold: 5,
    //     bulk_discount: 15.0,
    // }));
    // calculator.calculate(100.0);

    // let gateway: Box<dyn PaymentGateway> = match std::env::var("PAYMENT_GATEWAY")
    //     .unwrap_or(String::from("razorpay"))
    //     .as_str()
    // {
    //     "stripe"   => Box::new(StripeAdapter::new("sk_test_abc123xyz")),
    //     "paypal"   => Box::new(PayPalAdapter::new("AYSq3R...", true)),
    //     _          => Box::new(RazorpayAdapter::new("rzp_test_abc", "secret_xyz")),
    // };

    // let service = OrderService::new(gateway);

    // // Checkout
    // if let Ok(txn_id) = service.checkout(1001, 49900) {  // Rs. 499
    //     // Cancel if needed
    //     service.cancel_order(&txn_id);
    // }

    

    // let service = AlertService;

    // println!("=== Low urgency via Email ===");
    // let low_email = LowUrgencyNotification::new(
    //     Box::new(EmailSender::new("smtp.gmail.com"))
    // );
    // service.send_alert(&low_email, "user@example.com", "Your report is ready");

    // println!("\n=== High urgency via SMS ===");
    // let high_sms = HighUrgencyNotification::new(
    //     Box::new(SmsSender::new("twilio_key_abc123")),
    //     "Payments",
    // );
    // service.send_alert(&high_sms, "+919876543210", "Payment gateway is down");

    // println!("\n=== High urgency via Push ===");
    // let high_push = HighUrgencyNotification::new(
    //     Box::new(PushSender::new("dvion-prod")),
    //     "Security",
    // );
    // service.send_alert(&high_push, "device_token_xyz", "Unusual login detected");

    // println!("\n=== Critical — all channels at once ===");
    // let critical = CriticalNotification::new(vec![
    //     Box::new(EmailSender::new("smtp.gmail.com")),
    //     Box::new(SmsSender::new("twilio_key_abc123")),
    //     Box::new(PushSender::new("dvion-prod")),
    // ]);
    // service.send_alert(&critical, "admin@dvion.com", "Server is unreachable");
    
    // let pipeline = Pipeline::new()
    //     .add(Box::new(LoggingMiddleware))
    //     .add(Box::new(AuthMiddleware::new(vec!["token_alice", "token_bob"])))
    //     .add(Box::new(RateLimitMiddleware::new(2)))
    //     .add(Box::new(RouteHandler::new()));

    //     // Case 1: Valid request — passes through all middleware
    // let resp = pipeline.run(
    //     &Request::new("GET", "/users")
    //         .with_token("token_alice")
    //         .with_ip("10.0.0.1")
    // );
    // println!("  Response: {} — {}\n", resp.status, resp.body);

    // // Case 2: Missing token — rejected at Auth
    // let resp = pipeline.run(
    //     &Request::new("GET", "/users")
    //         .with_ip("10.0.0.2")
    // );
    // println!("  Response: {} — {}\n", resp.status, resp.body);

    // // Case 3: Invalid token — rejected at Auth
    // let resp = pipeline.run(
    //     &Request::new("GET", "/users")
    //         .with_token("hacker_token")
    //         .with_ip("10.0.0.3")
    // );
    // println!("  Response: {} — {}\n", resp.status, resp.body);

    //  for i in 1..=3 {
    //     println!("  -- attempt {} --", i);
    //     let resp = pipeline.run(
    //         &Request::new("GET", "/health")
    //             .with_token("token_bob")
    //             .with_ip("10.0.0.4")
    //     );
    //     println!("  Response: {} — {}\n", resp.status, resp.body);
    // }
    
     let tv     = Rc::new(RefCell::new(Television::new()));
    let mut remote = Remote::new();

    println!("--- Starting state ---");
    tv.borrow().show();

    println!("\n--- Press volume up x2 ---");
    remote.press(Box::new(VolumeUpCommand::new(Rc::clone(&tv), 1)));
    remote.press(Box::new(VolumeUpCommand::new(Rc::clone(&tv), 1)));
    tv.borrow().show();

    println!("\n--- Press mute ---");
    remote.press(Box::new(MuteCommand::new(Rc::clone(&tv))));
    tv.borrow().show();

    println!("\n--- Undo mute ---");
    remote.undo_last();
    tv.borrow().show();

    println!("\n--- Switch to channel 7 ---");
    remote.press(Box::new(ChannelCommand::new(Rc::clone(&tv), 7)));
    tv.borrow().show();

    println!("\n--- Switch to channel 20 ---");
    remote.press(Box::new(ChannelCommand::new(Rc::clone(&tv), 20)));
    tv.borrow().show();


    println!("\n--- Undo channel (go back to 7) ---");
    remote.undo_last();
    tv.borrow().show();

    println!("\n--- Undo channel again (go back to 1) ---");
    remote.undo_last();
    tv.borrow().show();

}

