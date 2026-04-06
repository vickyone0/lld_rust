mod defult_method;
mod dyn_trait;
mod trait_object;
mod stratergy_pattern;
mod adapter_pattern;

//use defult_method::*;
use defult_method::Describe;
//use dyn_trait::*;
use trait_object::*;
use stratergy_pattern::*;
use adapter_pattern::*;

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

    let gateway: Box<dyn PaymentGateway> = match std::env::var("PAYMENT_GATEWAY")
        .unwrap_or(String::from("razorpay"))
        .as_str()
    {
        "stripe"   => Box::new(StripeAdapter::new("sk_test_abc123xyz")),
        "paypal"   => Box::new(PayPalAdapter::new("AYSq3R...", true)),
        _          => Box::new(RazorpayAdapter::new("rzp_test_abc", "secret_xyz")),
    };

    let service = OrderService::new(gateway);

    // Checkout
    if let Ok(txn_id) = service.checkout(1001, 49900) {  // Rs. 499
        // Cancel if needed
        service.cancel_order(&txn_id);
    }
}

