pub mod enums;
pub mod structs;

use enums::order_types::OrderType;
use structs::order::Order;
use structs::stock::Stock;

/// This is a function that opens an order.
///
/// # Arguments
/// * `number` - An i32 that holds the number of shares of the order.
/// * `order_type` - An OrderType that holds the type of order.
/// * `stock_name` - A string slice that holds the name of the stock of the order.
/// * `open_price` - A float that holds the open price of the stock of the order.
/// * `stop_loss` - An Option that holds the stop loss of the stock of the order.
/// * `take_profit` - An Option that holds the take profit of the stock of the order.
///
/// # Returns
/// * An Order struct.
pub fn open_order(
    number: i32,
    order_type: OrderType,
    stock_name: &str,
    open_price: f32,
    stop_loss: Option<f32>,
    take_profit: Option<f32>,
) -> Order {
    println!("order for {} is being made", &stock_name);
    let mut stock: Stock = Stock::new(stock_name, open_price);
    match stop_loss {
        Some(value) => stock = stock.with_stop_loss(value),
        None => (),
    }
    match take_profit {
        Some(value) => stock = stock.with_take_profit(value),
        None => (),
    }
    return Order::new(stock, number, order_type);
}

/// This is a function that closes an order.
///     
/// # Arguments
/// * `order` - An Order that holds the order that is being closed.
///
/// # Returns
/// * A float that holds the current profit of the order.
pub fn close_order(order: Order) -> f32 {
    println!("order for {} is being closed", &order.stock.name);
    return order.current_profit();
}
