use std::env;
use seikyo_notifier::seikyo_client::point;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [token]", args[0]);
        return;
    }
    let token = &args[1];
    let amount = point::get_prepaid_amount(token).unwrap();
    println!("The amount is {} yen", amount);
}
