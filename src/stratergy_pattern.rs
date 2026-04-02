pub trait PrincingStrategy {
    fn calculate(&self, base_price: f64) -> f64;
    fn name(&self) -> &str;
}


pub struct RegularPrice;

impl PrincingStrategy for RegularPrice {
    fn calculate(&self, base_price: f64) -> f64 {
        base_price
    }
    fn name(&self) -> &str {
        "Regular Price"
    }
}

pub struct DiscountPrice {
    pub discount_percent: f64,
}

impl PrincingStrategy for DiscountPrice {
 fn calculate(&self, base_price: f64) -> f64 {
        base_price * (1.0 - self.discount_percent/100.0)
    }
     fn name(&self) -> &str {
        "Discount Price"
    }
}


pub struct SeasonalPrice {
   pub  multiplier: f64,
}

impl PrincingStrategy for SeasonalPrice {
    fn calculate(&self, base_price: f64) -> f64 {
        base_price * self.multiplier
    }
    fn name(&self) -> &str {
        "Seasonal Price"
    }
}


pub struct BulkPrice {
   pub  quantity: u32,
    pub tier_threshold: u32,
    pub bulk_discount: f64,
}

impl PrincingStrategy for BulkPrice {
    fn calculate(&self, base_price: f64) -> f64 {
        if self.quantity >= self.tier_threshold {
            base_price * (1.0 - self.bulk_discount/100.0)
        } else {
            base_price
        }
    }
    fn name(&self) -> &str {
        "Bulk Price"
    }
}


//Hold and uses a strategy

pub struct PriceCalculator {
    strategy: Box<dyn PrincingStrategy>,
}

impl PriceCalculator {
    pub fn new(strategy: Box<dyn PrincingStrategy>) -> Self {
        PriceCalculator { strategy }
    }

    //swap strategy at runtime
    pub fn set_strategy(&mut self, strategy: Box<dyn PrincingStrategy>) {
        self.strategy = strategy;
    }

    pub fn calculate(&self, base_price: f64) -> f64 {

        let final_price = self.strategy.calculate(base_price);
        println!(
            " [{}] base = {:.2}, final = {:.2}",
            self.strategy.name(), base_price, final_price
        );
        final_price
    }
}


