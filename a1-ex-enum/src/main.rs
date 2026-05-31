enum OrderStatus {
    Pending,
    Approved,
    Shipped,
    Delivered,
    Cancelled,
    Returned,
}

fn print_status(status: OrderStatus) {
    match status {
        OrderStatus::Pending => {
            println!("ההזמנה ממתינה לאישור");
        }
        OrderStatus::Approved => {
            println!("ההזמנה אושרה");
        }
        OrderStatus::Shipped => {
            println!("ההזמנה נשלחה");
        }
        OrderStatus::Delivered => {
            println!("ההזמנה נמסרה");
        }
        OrderStatus::Cancelled => {
            println!("ההזמנה בוטלה");
        }
        OrderStatus::Returned => {
            println!("ההזמנה הוחזרה");
        }
    }
}

fn main() {
    let order1 = OrderStatus::Pending;
    let order2 = OrderStatus::Shipped;
    let order3 = OrderStatus::Delivered;
    let order4 = OrderStatus::Cancelled;
    let order5 = OrderStatus::Returned;

    print_status(order1);
    print_status(order2);
    print_status(order3);
    print_status(order4);
    print_status(order5);
}