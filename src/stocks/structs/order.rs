use super::super::enums::order_types::OrderType;
use super::stock::Stock;
use chrono;

/// This is an enum that represents the type of order.
///     
/// # Fields
/// * date - A DateTime that holds the date of the order.
/// * stock - A Stock that holds the stock of the order.
/// * number - An i32 that holds the number of shares of the order.
/// * order_type - An OrderType that holds the type of order.
pub struct Order {
    pub date: chrono::DateTime<chrono::Local>,
    pub stock: Stock,
    pub number: i32,
    pub order_type: OrderType,
}

impl Order {
    /// The constructor for the Order struct.
    ///     
    /// # Arguments
    /// * `stock` - A Stock that holds the stock of the order.
    /// * `number` - An i32 that holds the number of shares of the order.
    /// * `order_type` - An OrderType that holds the type of order.
    ///
    /// # Returns
    /// * An Order struct.
    pub fn new(stock: Stock, number: i32, order_type: OrderType) -> Order {
        let today: chrono::DateTime<chrono::Local> = chrono::Local::now();
        return Order {
            date: today,
            stock,
            number,
            order_type,
        };
    }

    /// Calculates the current profit of the order.
    ///
    /// # Returns
    /// * A float that holds the current profit of the order.
    pub fn current_profit(&self) -> f32 {
        let current_price: f32 = self.stock.current_price;
        let open_price: f32 = self.stock.open_price;

        match self.order_type {
            OrderType::Long => current_price - open_price,
            OrderType::Short => open_price - current_price,
        }
    }

    /// Calculates the current value of the order.
    ///     
    /// # Returns
    /// * A float that holds the current value of the order.
    pub fn current_value(&self) -> f32 {
        return self.stock.current_price * self.number as f32;
    }
}
