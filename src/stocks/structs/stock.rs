/// This is a struct that represents a stock.
///
/// # Fields
/// * name - A string that holds the name of the stock.
/// * open_price - A float that holds the price of the stock when it was opened.
/// * stop_loss - A float that holds the stop loss price of the stock.
/// * take_profit - A float that holds the take profit price of the stock.
/// * current_price - A float that holds the current price of the stock.
pub struct Stock {
    pub name: String,
    pub open_price: f32,
    pub stop_loss: f32,
    pub take_profit: f32,
    pub current_price: f32,
}

impl Stock {
    /// The constructor for the Stock struct.
    ///
    /// # Arguments
    /// * `stock_name` - A string slice that holds the name of the stock.
    /// * `price` - A float that holds the price of the stock when it was opened.
    ///
    /// # Returns
    /// * A Stock struct.
    pub fn new(stock_name: &str, price: f32) -> Stock {
        Stock {
            name: String::from(stock_name),
            open_price: price,
            stop_loss: 0.0,
            take_profit: 0.0,
            current_price: 0.0,
        }
    }
    /// Adds a `stop_loss` price to the Stock struct.
    ///
    /// # Arguments
    /// * `value` - A float that holds the stop loss price of the stock.
    ///
    /// # Returns
    /// * A Stock struct.
    pub fn with_stop_loss(mut self, value: f32) -> Stock {
        self.stop_loss = value;
        return self;
    }

    /// Adds a `take_profit` price to the Stock struct.
    ///
    /// # Arguments
    /// * `value` - A float that holds the take profit price of the stock.
    ///
    /// # Returns
    /// * A Stock struct.
    pub fn with_take_profit(mut self, value: f32) -> Stock {
        self.take_profit = value;
        return self;
    }

    /// Adds a `current_price` to the Stock struct.
    ///
    /// # Arguments
    /// * `value` - A float that holds the current price of the stock.
    pub fn update_price(&mut self, value: f32) {
        self.current_price = value;
    }
}
