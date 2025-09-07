//in the name of God//

use chrono::Local;

use crate::model::{
    All, Category, ErrorType, Item, Order, OrderStatus, Resturant, ResturantOwner, User, UserOrder,
};

use std::{fs, io};

use colored::Colorize;

pub trait UserCapabilities {
    fn show_resturants_and_order(&mut self) -> Result<String, ErrorType>;
    fn show_resturant_menu(&mut self, res_name: &str) -> Result<String, ErrorType>;
    fn order(&self, res: &mut Resturant) -> Result<String, ErrorType>;
    fn modify(&self, res: &mut Resturant) -> Result<String, ErrorType>;
    fn show_date_time(&self) -> Result<(), ErrorType>;
    fn show_resturants_with_discount(&self) -> Result<(), ErrorType>;
    fn change_order_staus(&self) -> Result<String, ErrorType>;
}

impl UserCapabilities for User {
    fn show_resturants_and_order(&mut self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let arr = if str.trim().len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no resturants available".to_string(),
            ));
        } else {
            let res = match serde_json::from_str::<Vec<Resturant>>(&str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            };
            res
        };

        for res in arr.iter() {
            println!(
                "{}",
                format!("Resturant's name : {}", res.name).bright_purple()
            );
            match res.category {
                Category::FastFood => {
                    println!("{}", "Resturant's category : Fast Food".bright_cyan())
                }
                Category::TraditionalFood => println!(
                    "{}",
                    "Resturant's category : Traditional Food".bright_cyan()
                ),
                Category::Dessert => {
                    println!("{}", "Resturant's category : Dessert".bright_cyan());
                }
                Category::Persian => {
                    println!("{}", "Resturant's category : Persian".bright_cyan());
                }
                Category::SeaFood => {
                    println!("{}", "Resturant's category : Sea Food".bright_cyan());
                }
                Category::Vegan => {
                    println!("{}", "Resturant's category : Sea Food".bright_cyan());
                }
            }
            println!("{}", "------------------------".bright_white())
        }
        println!("{}", "1 : Choose resturant -- 2 : Back".bright_yellow());
        let choice = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u32>() {
                Ok(num) => break num,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            }
        };

        let last_message = match choice {
            1 => {
                println!(
                    "{}",
                    "Please enter the name of the resturant you want to see its menu".bright_blue()
                );
                let mut res_name = String::new();
                io::stdin().read_line(&mut res_name).unwrap();

                match self.show_resturant_menu(res_name.trim()) {
                    Ok(val) => val,

                    Err(e) => return Err(e),
                }
            }

            2 => "back".to_string(),

            _ => {
                return Err(ErrorType::InvalidCommand(
                    "This command is invalid".to_string(),
                ))
            }
        };

        Ok(last_message)
    }

    fn show_resturant_menu(&mut self, res_name: &str) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr = if str.trim().len() == 0 {
            Vec::new()
        } else {
            let holder = match serde_json::from_str::<Vec<Resturant>>(&str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            };
            holder
        };

        if arr.len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no resturant on our list".to_string(),
            ));
        }

        let result = arr.iter_mut().find(|item| item.name == res_name);

        let wanted_resturant = match result {
            Some(val) => val,

            None => {
                return Err(ErrorType::NotFound(
                    "This resturant is not existed".to_string(),
                ))
            }
        };

        let _ = match wanted_resturant.show_menu() {
            Ok(_) => {}

            Err(e) => return Err(e),
        };

        println!(
            "{}",
            "1 : Order -- 2 : Modify(Add or remove) -- 3 : Back".bright_yellow()
        );

        let choice = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u32>() {
                Ok(num) => break num,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            }
        };

        let last_message;

        let _ = match choice {
            1 => {
                last_message = match self.order(wanted_resturant) {
                    Ok(val) => val,

                    Err(e) => return Err(e),
                };
            }

            2 => {
                last_message = match self.modify(wanted_resturant) {
                    Ok(val) => val,

                    Err(e) => return Err(e),
                };
            }

            3 => last_message = "back".to_string(),

            _ => {
                return Err(ErrorType::InvalidCommand(
                    "This command is invalid".to_string(),
                ))
            }
        };

        let resturants = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonEncode("Cannot encode to json".to_string())),
        };

        match fs::write("data/resturant.json", resturants) {
            Ok(_) => return Ok(last_message),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn order(&self, res: &mut Resturant) -> Result<String, ErrorType> {
        let order_str = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut order_arr = if order_str.trim().len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<Order>>(&order_str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        let user_str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        };

        let mut user_arr = if user_str.trim().len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<All>>(&user_str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        let time = Local::now();
        let mut order = Order {
            username: self.username.clone(),
            resturant: res.name.clone(),
            items: Vec::new(),
            total_price: 0.0,
            datetime: format!("{}", time.format("%Y-%m-%d %H:%M:%S")),
            status: OrderStatus::Preparing,
        };

        let mut user_order = UserOrder {
            resturant: res.name.clone(),
            items: Vec::new(),
            total_price: 0.0,
            datetime: format!("{}", time.format("%Y-%m-%d %H:%M:%S")),
            status: OrderStatus::Preparing,
        };

        'outer: loop {
            let wanted_item = loop {
                println!(
                    "{}{}",
                    "Enter the name of the product".bright_blue(),
                    "(Enter (done) when you finish ordering)".bright_yellow()
                );
                let mut prod = String::new();
                io::stdin().read_line(&mut prod).unwrap();

                if prod.trim() == "done" || prod.trim() == "Done" {
                    break 'outer;
                }

                let prod = res.menu.iter_mut().find(|item| item.name == prod.trim());

                match prod {
                    Some(val) => break val,

                    None => {
                        println!("{}", "This item is not existed".bright_red());
                        continue;
                    }
                }
            };

            loop {
                println!("{}", "Enter the quantity of your purchase".bright_blue());
                let quantity = loop {
                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).unwrap();
                    match quantity.trim().parse::<u32>() {
                        Ok(val) => break val,

                        Err(_) => {
                            println!("{}", "Invalid number".bright_red());
                            continue;
                        }
                    }
                };

                if wanted_item.quantity >= quantity {
                    let holder = Item {
                        name: wanted_item.name.clone(),
                        price: wanted_item.price
                            - (wanted_item.price * res.discount as f64) / 100.0,
                        quantity,
                    };

                    println!(
                        "{}",
                        format!("The item named {} added to your purchase list", holder.name)
                            .bright_green()
                    );
                    user_order.items.push(holder.clone());
                    order.items.push(holder);

                    wanted_item.quantity -= quantity;

                    break;
                } else {
                    println!(
                        "{}",
                        "Sorry there is no enough quantity of this food".bright_red()
                    );
                    println!(
                        "{}",
                        format!("We only have {} of it", wanted_item.quantity).bright_red()
                    );
                    continue 'outer;
                }
            }
        }

        let mut sum = 0.0;

        user_order.items.iter().for_each(|item| {
            sum += item.price * item.quantity as f64;
        });

        println!("{}", "Here is your order information".bright_purple());
        println!(
            "{}",
            format!("Resturant's name : {}", order.resturant).bright_purple()
        );
        println!("{}", format!("Items : {:#?}", order.items).bright_purple());
        println!("{}", format!("Total price : {}", sum).bright_purple());

        println!("{}", "Would you like to confirm the order?".bright_yellow());

        println!("{}", "1 : Yes -- 2 : No".bright_yellow());

        let confirm_check = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u32>() {
                Ok(val) => break val,

                Err(_) => {
                    println!("{}", "Invalid number".bright_yellow());
                    continue;
                }
            }
        };

        let _ = match confirm_check {
            1 => {}

            2 => {
                return Err(ErrorType::CanceledOrder(
                    "Sure! We will cancel your order".to_string(),
                ));
            }

            _ => return Err(ErrorType::InvalidCommand("Invalid command".to_string())),
        };

        user_order.total_price = sum;
        order.total_price = sum;

        let result = {
            let res = user_arr.iter_mut().find(|item| {
                if let All::User(user) = item {
                    user.username == self.username
                } else {
                    false
                }
            });

            match res {
                Some(val) => val,
                None => {
                    return Err(ErrorType::NotFound(
                        "Cannot find the item on your purchase list".to_string(),
                    ))
                }
            }
        };

        let result = match result {
            All::User(val) => val,

            _ => {
                return Err(ErrorType::NGH);
            }
        };

        order_arr.push(order);
        result.order.push(user_order);

        let order_arr = match serde_json::to_string_pretty(&order_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let user_arr = match serde_json::to_string_pretty(&user_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let _ = match fs::write("data/order.json", order_arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        };

        let _ = match fs::write("data/user.json", user_arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        };

        return Ok("Your order has been successfully placed".to_string());
    }

    fn modify(&self, res: &mut Resturant) -> Result<String, ErrorType> {
        let order_arr = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut order_arr = if order_arr.len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<Order>>(&order_arr) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        let user_arr = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut user_arr = if user_arr.len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<All>>(&user_arr) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        let mut ord_num = 1;
        order_arr
            .iter()
            .filter(|order| order.username == self.username && order.resturant == res.name)
            .for_each(|order| {
                println!(
                    "{}",
                    format!("resturant's name : {}", order.resturant).bright_cyan()
                );
                println!("{}", format!("Items : {:#?}", order.items).bright_cyan());
                println!("{}", format!("Datetime : {}", order.datetime).bright_cyan());
                println!("{}", format!("Status : {}", order.status).bright_cyan());
                println!("{}", format!("Order number : {}", ord_num).bright_cyan());
                println!("{}", "-----------------------".bright_white());
                ord_num += 1;
            });

        if ord_num == 1 {
            return Err(ErrorType::WrongResturant("Sorry you havn not any purchases from this resturant. Try another resturant or order".to_string()));
        }

        println!("{}", "Enter your command : ".bright_blue());
        println!(
            "{}",
            "1 : Add to purchase list -- 2 : Remove ".bright_yellow()
        );

        let command = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u32>() {
                Ok(val) => break val,

                Err(_) => {
                    println!("{}", "invalid number".bright_red());
                    continue;
                }
            }
        };

        let wanted_user_order = {
            let res = user_arr.iter_mut().find(|item| {
                if let All::User(user) = item {
                    user.username == self.username
                } else {
                    false
                }
            });

            match res {
                Some(val) => val,
                None => {
                    return Err(ErrorType::NotFound(
                        "Cannot find the item on your purchase list".to_string(),
                    ))
                }
            }
        };

        let wanted_user = match wanted_user_order {
            All::User(val) => val,

            _ => return Err(ErrorType::NGH),
        };

        if command == 1 {
            println!("{}", "Enter the number of the order".bright_blue());

            let order_num = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<usize>() {
                    Ok(val) => {
                        if val > wanted_user.order.len() {
                            println!("{}", "Order number is out of bound!".bright_red());
                            continue;
                        }
                        break val;
                    }

                    Err(_) => {
                        println!("{}", "Invalid number".bright_red());
                    }
                }
            };

            println!(
                "{}",
                "Please enter the name of the item you want to add".bright_blue()
            );

            let mut item_name = String::new();
            io::stdin().read_line(&mut item_name).unwrap();

            let item_name = item_name.trim();

            let wanted_user_order = &mut wanted_user.order[order_num - 1];

            let result: Vec<&mut Order> = order_arr
                .iter_mut()
                .filter(|order| order.username == self.username && order.resturant == res.name)
                .collect();

            if result.len() == 0 {
                return Err(ErrorType::WrongResturant("Sorry you havn not any purchases from this resturant. Try another resturant or order".to_string()));
            }

            let prod = res.menu.iter_mut().find(|holder| holder.name == item_name);

            let prod = match prod {
                Some(val) => val,

                None => {
                    return Err(ErrorType::NotFound("This item is not found".to_string()));
                }
            };

            println!(
                "{}",
                "Please enter the quantity you want to add".bright_blue()
            );

            let quantity = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<u32>() {
                    Ok(val) => break val,

                    Err(_) => {
                        println!("{}", "Invalid number".bright_red());
                        continue;
                    }
                }
            };

            if quantity <= prod.quantity {
                let mut wanted_order = None;
                for order in result {
                    let mut res1 = None;
                    for (index, item) in order.items.iter().enumerate() {
                        if index >= wanted_user_order.items.len() {
                            res1 = None;
                            break;
                        }
                        if wanted_user_order.items[index] == *item {
                        } else {
                            res1 = None;
                            break;
                        }

                        res1 = Some(());
                    }

                    match res1 {
                        Some(_) => wanted_order = Some(order),

                        None => {}
                    }
                }

                let wanted_order = match wanted_order {
                    Some(val) => val,

                    None => {
                        return Err(ErrorType::NotFound(
                            "This item did not exist on your purchase list".to_string(),
                        ))
                    }
                };

                let holder = Item {
                    name: item_name.to_string(),
                    price: prod.price - (prod.price * res.discount as f64) / 100.0,
                    quantity,
                };

                wanted_order.items.push(holder.clone());
                wanted_user_order.items.push(holder);

                prod.quantity -= quantity;

                let time = Local::now();
                wanted_user_order.datetime = format!("{}", time.format("%Y-%m-%d %H:%M:%S"));
                wanted_order.datetime = format!("{}", time.format("%Y-%m-%d %H:%M:%S"));

                let mut sum = 0.0;

                wanted_user_order
                    .items
                    .iter()
                    .for_each(|item| sum += item.price * item.quantity as f64);

                wanted_user_order.total_price = sum;
                wanted_order.total_price = sum;
                println!(
                    "{}",
                    format!(
                        "The item named {} has been added to the purchase list seccessfully ",
                        item_name
                    )
                    .bright_green()
                );
            } else {
                return Err(ErrorType::HigherQuantity(
                    "Sorry there is not enough of this food for you to order".to_string(),
                ));
            }

            let order_arr = match serde_json::to_string_pretty(&order_arr) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonEncode(
                        "Cannot encode it to json".to_string(),
                    ))
                }
            };

            let user_arr = match serde_json::to_string_pretty(&user_arr) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonEncode(
                        "Cannot encode it to json".to_string(),
                    ))
                }
            };

            let _ = match fs::write("data/order.json", order_arr) {
                Ok(_) => {}

                Err(_) => {
                    return Err(ErrorType::WriteOnFile(
                        "Cannot write on this file at this moment".to_string(),
                    ))
                }
            };

            let _ = match fs::write("data/user.json", user_arr) {
                Ok(_) => {}

                Err(_) => {
                    return Err(ErrorType::WriteOnFile(
                        "Cannot write on this file at this moment".to_string(),
                    ))
                }
            };
        } else if command == 2 {
            println!("{}", "Enter the number of the order".bright_blue());

            let order_num = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<usize>() {
                    Ok(val) => {
                        if val > wanted_user.order.len() {
                            println!("{}", "Order number is out of bound!".blue());
                            continue;
                        }
                        break val;
                    }

                    Err(_) => {
                        println!("{}", "Invalid number".bright_red());
                    }
                }
            };

            println!(
                "{}",
                "Please enter the name of the item you want to delete".bright_blue()
            );

            let mut item_name = String::new();
            io::stdin().read_line(&mut item_name).unwrap();

            let item_name = item_name.trim();

            let wanted_user_order = &mut wanted_user.order[order_num - 1];

            let result: Vec<&mut Order> = order_arr
                .iter_mut()
                .filter(|order| order.username == self.username && order.resturant == res.name)
                .collect();

            if result.len() == 0 {
                return Err(ErrorType::WrongResturant("Sorry you havn not any purchases from this resturant. Try another resturant or order".to_string()));
            }

            let (item_user_index, item_user) = {
                let res = wanted_user_order
                    .items
                    .iter_mut()
                    .enumerate()
                    .find(|item| item.1.name == item_name);

                match res {
                    Some(val) => val,

                    None => {
                        return Err(ErrorType::NotFound(
                            "Cannot find the item on your purchase list".to_string(),
                        ))
                    }
                }
            };

            println!(
                "{}",
                "Please enter the quantity you want to remove".bright_blue()
            );

            let quantity = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<u32>() {
                    Ok(val) => break val,

                    Err(_) => {
                        println!("{}", "Invalid number".bright_red());
                        continue;
                    }
                }
            };

            if quantity <= item_user.quantity {
                let mut wanted_order = None;
                for order in result {
                    let res1 = order.items.iter().find(|item| *item == item_user);

                    match res1 {
                        Some(_) => wanted_order = Some(order),

                        None => {}
                    }
                }

                let wanted_order = match wanted_order {
                    Some(val) => val,

                    None => {
                        return Err(ErrorType::NotFound(
                            "This item did not exist on your purchase list".to_string(),
                        ))
                    }
                };

                let item = wanted_order
                    .items
                    .iter_mut()
                    .enumerate()
                    .find(|item| item.1 == item_user);

                let (item_index, item) = match item {
                    Some(val) => val,

                    None => {
                        return Err(ErrorType::NotFound(
                            "This item did not exist on your purchase list".to_string(),
                        ))
                    }
                };

                item.quantity -= quantity;
                item_user.quantity -= quantity;

                let prod = res.menu.iter_mut().find(|holder| holder.name == item.name);

                let prod = match prod {
                    Some(val) => val,

                    None => {
                        return Err(ErrorType::NotFound("This item is not found".to_string()));
                    }
                };
                prod.quantity += quantity;
                if item.quantity == 0 {
                    wanted_order.items.remove(item_index);
                    wanted_user_order.items.remove(item_user_index);
                }
                let time = Local::now();
                wanted_user_order.datetime = format!("{}", time.format("%Y-%m-%d %H:%M:%S"));
                wanted_order.datetime = format!("{}", time.format("%Y-%m-%d %H:%M:%S"));

                let mut sum = 0.0;

                wanted_user_order.items.iter().for_each(|item| {
                    sum += (item.price - (item.price * res.discount as f64) / 100.0)
                        * item.quantity as f64
                });

                wanted_user_order.total_price = sum;
                wanted_order.total_price = wanted_user_order.total_price;
                println!(
                    "{}",
                    format!(
                        "The item named {} has been removed from the purchase list successfully",
                        item_name
                    )
                    .bright_green()
                );
            } else {
                return Err(ErrorType::HigherQuantity(
                    "You do not have this number of this food to remove!!".to_string(),
                ));
            }

            let order_arr = match serde_json::to_string_pretty(&order_arr) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonEncode(
                        "Cannot encode it to json".to_string(),
                    ))
                }
            };

            let user_arr = match serde_json::to_string_pretty(&user_arr) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonEncode(
                        "Cannot encode it to json".to_string(),
                    ))
                }
            };

            let _ = match fs::write("data/order.json", order_arr) {
                Ok(_) => {}

                Err(_) => {
                    return Err(ErrorType::WriteOnFile(
                        "Cannot write on this file at this moment".to_string(),
                    ))
                }
            };

            let _ = match fs::write("data/user.json", user_arr) {
                Ok(_) => {}

                Err(_) => {
                    return Err(ErrorType::WriteOnFile(
                        "Cannot write on this file at this moment".to_string(),
                    ))
                }
            };
        }

        Ok("Your purchase list has been changed successfully".to_string())
    }

    fn show_date_time(&self) -> Result<(), ErrorType> {
        let order_str = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        if order_str.trim().len() == 0 {
            return Err(ErrorType::EmptyFile(
                "You haven't ordered anything yet.".to_string(),
            ));
        }

        let order_arr = match serde_json::from_str::<Vec<Order>>(&order_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let arr_for_print = order_arr
            .iter()
            .filter(|order| order.username == self.username)
            .collect::<Vec<&Order>>();

        if arr_for_print.len() == 0 {
            return Err(ErrorType::NotFound(
                "No order found for your account!".to_string(),
            ));
        }

        println!("{}", "All orders : ".bright_blue());
        println!(
            "{}",
            format!("Your username : {}", self.username).bright_cyan()
        );
        for order in arr_for_print {
            println!(
                "{}",
                format!("Resturant's name : {}", order.resturant).bright_cyan()
            );
            println!("{}", format!("Items : {:#?}", order.items).bright_cyan());
            println!("");
            println!(
                "{}",
                format!("Total price for this order : {}", order.total_price).bright_cyan()
            );
            println!("");
            println!(
                "{}",
                format!("Date time : {}", order.datetime).bright_cyan()
            );
            println!("{}", format!("Status : {}", order.status).bright_cyan());
            println!("{}", "------------------------------------".bright_white())
        }

        return Ok(());
    }

    fn show_resturants_with_discount(&self) -> Result<(), ErrorType> {
        let res_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        if res_str.trim().len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no resturant on this app!!".to_string(),
            ));
        }

        let res_arr = match serde_json::from_str::<Vec<Resturant>>(&res_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let mut dis_rests: Vec<&Resturant> =
            res_arr.iter().filter(|res| res.discount != 0).collect();

        if dis_rests.len() == 0 {
            return Err(ErrorType::NotFound(
                "There is no resturant with discount".to_string(),
            ));
        }

        dis_rests.sort_by_key(|res| res.discount);

        println!("{}", "Here is all resturants with discount".bright_purple());

        for res in dis_rests {
            println!("{}", "Name : ".bright_purple());
            println!("{}", res.name.bright_cyan());
            println!("{}", "Category : ".bright_purple());
            println!("{}", format!("{}", res.category).bright_cyan());
            println!("{}", "Discount percentage : ".bright_purple());
            println!("{}%", res.discount);
            println!("-------------------------------------------------");
        }

        Ok(())
    }

    fn change_order_staus(&self) -> Result<String, ErrorType> {
        let order_arr = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut order_arr = if order_arr.len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<Order>>(&order_arr) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        let user_arr = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut user_arr = if user_arr.len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<All>>(&user_arr) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        let mut ord_num = 1;
        order_arr
            .iter()
            .filter(|order| order.username == self.username)
            .for_each(|order| {
                println!(
                    "{}",
                    format!("resturant's name : {}", order.resturant).bright_cyan()
                );
                println!("{}", format!("Items : {:#?}", order.items).bright_cyan());
                println!("{}", format!("Datetime : {}", order.datetime).bright_cyan());
                println!("{}", format!("Status : {}", order.status).bright_cyan());
                println!("{}", format!("Order number : {}", ord_num).bright_cyan());
                println!("{}", "-----------------------".bright_white());
                ord_num += 1;
            });

        println!("{}", "Enter the number of the order".bright_blue());

        let order_num = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(val) => {
                    if val > order_arr.len() {
                        println!("{}", "Order number is out of bound!".bright_red());
                        continue;
                    }
                    break val;
                }

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                }
            }
        };

        println!("{}", "Enter new status : ".bright_blue());
        println!(
            "{}",
            "1 : Preparing -- 2 : Delivered -- 3 : Canceled ".bright_yellow()
        );

        let command = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u32>() {
                Ok(val) => break val,

                Err(_) => {
                    println!("{}", "invalid number".bright_red());
                    continue;
                }
            }
        };
        let wanted_user_order = {
            let res = user_arr.iter_mut().find(|item| {
                if let All::User(user) = item {
                    user.username == self.username
                } else {
                    false
                }
            });

            match res {
                Some(val) => val,
                None => {
                    return Err(ErrorType::NotFound(
                        "Cannot find the item on your purchase list".to_string(),
                    ))
                }
            }
        };

        let wanted_user = match wanted_user_order {
            All::User(val) => val,

            _ => return Err(ErrorType::NGH),
        };

        let status = match command {
            1 => {
                order_arr[order_num - 1].status = OrderStatus::Preparing;
                OrderStatus::Preparing
            }

            2 => {
                order_arr[order_num - 1].status = OrderStatus::Delivered;
                OrderStatus::Delivered
            }

            3 => {
                order_arr[order_num - 1].status = OrderStatus::Canceled;
                OrderStatus::Canceled
            }

            _ => {
                return Err(ErrorType::InvalidCommand(
                    "This command is not valid".to_string(),
                ));
            }
        };
        let wanted_order = wanted_user.order.iter_mut().find(|order| {
            order.items == order_arr[order_num - 1].items
                && order_arr[order_num - 1].resturant == order.resturant
        });

        match wanted_order {
            Some(order) => order.status = status,

            None => return Err(ErrorType::NGH),
        }

        let order_arr = match serde_json::to_string_pretty(&order_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let user_arr = match serde_json::to_string_pretty(&user_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let _ = match fs::write("data/order.json", order_arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        };

        let _ = match fs::write("data/user.json", user_arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        };

        Ok("The status has been cahnged succesfully".to_string())
    }
}

impl Resturant {
    pub fn show_menu(&self) -> Result<(), ErrorType> {
        let len = self.menu.len();
        if len == 0 {
            return Err(ErrorType::EmptyFile(
                "There is no food on this resturant's menu!!".to_string(),
            ));
        }

        println!(
            "{}",
            "----------------------------------------------------------------".bright_white()
        );
        for i in (1..len).step_by(2) {
            if self.menu[i - 1].quantity == 0 {
                if self.discount != 0 {
                    println!(
                        "\x1b[9m{} ---- {}\x1b[0m                  {} ---- {}",
                        self.menu[i - 1].name,
                        self.menu[i - 1].price,
                        self.menu[i].name,
                        self.menu[i].price
                    );
                } else {
                    println!(
                        "\x1b[9m{} ---- {}\x1b[0m          {} ---- {}",
                        self.menu[i - 1].name,
                        self.menu[i - 1].price,
                        self.menu[i].name,
                        self.menu[i].price
                    );
                }
            } else if self.menu[i].quantity == 0 {
                println!(
                    "{} ---- {}          \x1b[9m{} ---- {}\x1b[0m",
                    self.menu[i - 1].name,
                    self.menu[i - 1].price,
                    self.menu[i].name,
                    self.menu[i].price
                );
            } else if self.menu[i].quantity == 0 && self.menu[i - 1].quantity == 0 {
                if self.discount != 0 {
                    println!(
                        "\x1b[9m{} ---- {}                  {} ---- {}\x1b[0m",
                        self.menu[i - 1].name,
                        self.menu[i - 1].price,
                        self.menu[i].name,
                        self.menu[i].price
                    );
                } else {
                    println!(
                        "\x1b[9m{} ---- {}          {} ---- {}\x1b[0m",
                        self.menu[i - 1].name,
                        self.menu[i - 1].price,
                        self.menu[i].name,
                        self.menu[i].price
                    );
                }
            } else if self.discount != 0 {
                println!(
                    "{} ---- \x1b[9m{}\x1b[0m --> {}          {} ---- \x1b[9m{}\x1b[0m --> {}",
                    self.menu[i - 1].name,
                    self.menu[i - 1].price,
                    self.menu[i - 1].price
                        - (self.menu[i - 1].price * self.discount as f64) / 100.0,
                    self.menu[i].name,
                    self.menu[i].price,
                    self.menu[i].price - (self.menu[i].price * self.discount as f64) / 100.0
                );
            } else {
                println!(
                    "{} ---- {}          {} ---- {}",
                    self.menu[i - 1].name.trim(),
                    self.menu[i - 1].price,
                    self.menu[i].name.trim(),
                    self.menu[i].price
                );
            }
        }
        if len % 2 != 0 {
            if self.menu[len - 1].quantity == 0 {
                println!(
                    "\x1b[9m{} ---- {}\x1b[0m",
                    self.menu[len - 1].name,
                    self.menu[len - 1].price
                );
            } else if self.discount != 0 {
                println!(
                    "{} ---- \x1b[9m{}\x1b[0m --> {}",
                    self.menu[len - 1].name,
                    self.menu[len - 1].price,
                    self.menu[len - 1].price
                        - (self.menu[len - 1].price * self.discount as f64) / 100.0,
                );
            } else {
                println!(
                    "{} ---- {}",
                    self.menu[len - 1].name,
                    self.menu[len - 1].price,
                );
            }
        }

        println!(
            "{}",
            "----------------------------------------------------------------".bright_white()
        );
        Ok(())
    }
}

pub trait ROCapabilities {
    //ResturantOwnerCapabilities

    fn add_item_to_menu(&self) -> Result<String, ErrorType>;
    fn delete_from_menu(&self) -> Result<String, ErrorType>;
    fn show_all_order(&self) -> Result<(), ErrorType>;
    fn show_all_order_datetime(&self) -> Result<(), ErrorType>;
    fn show_menu(&self) -> Result<(), ErrorType>;
    fn add_discount_to_resturant(&self) -> Result<String, ErrorType>;
    fn change_discount(&self) -> Result<String, ErrorType>;
    fn remove_discount(&self) -> Result<String, ErrorType>;
    fn change_price(&self) -> Result<String, ErrorType>;
    fn add_quantity(&self) -> Result<String, ErrorType>;
    fn change_order_status(&self) -> Result<String, ErrorType>;
}

impl ROCapabilities for ResturantOwner {
    fn add_item_to_menu(&self) -> Result<String, ErrorType> {
        let res_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut res_arr = match serde_json::from_str::<Vec<Resturant>>(&res_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let my_res = res_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        if my_res.menu.len() != 0 {
            println!("{}", "Here is your menu".bright_purple());
        }
        let _ = match my_res.show_menu() {
            Ok(_) => {}

            Err(_) => {}
        };

        loop {
            println!(
                "{}",
                "Enter (done) when you finish entering items".bright_blue()
            );

            println!("{}", "Please enter the name of the item".bright_blue());
            let mut item_name = String::new();
            io::stdin().read_line(&mut item_name).unwrap();

            if item_name.trim() == "done" || item_name.trim() == "Done" {
                break;
            }
            println!("{}", "Please enter the price of the item".bright_blue());
            let price = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<f64>() {
                    Ok(val) => break val,

                    Err(_) => {
                        println!("{}", "Invalid number".bright_red());
                        continue;
                    }
                }
            };

            println!("{}", "Please enter the quantity of the item".bright_blue());
            let quantity = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                match input.trim().parse::<u32>() {
                    Ok(val) => break val,

                    Err(_) => {
                        println!("{}", "Invalid number".bright_red());
                        continue;
                    }
                }
            };

            let holder = Item {
                name: item_name.trim().to_string(),
                price,
                quantity,
            };

            my_res.menu.push(holder);
            println!(
                "{}",
                format!(
                    "The item named {} has been added from the menu successfully",
                    item_name
                )
                .bright_green()
            );
        }

        let res_arr = match serde_json::to_string_pretty(&res_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_arr) {
            Ok(_) => return Ok("Your menu has been changed successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn delete_from_menu(&self) -> Result<String, ErrorType> {
        let res_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut res_arr = match serde_json::from_str::<Vec<Resturant>>(&res_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let my_res = res_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        println!("{}", "Here is your menu".bright_purple());
        let _ = match my_res.show_menu() {
            Ok(_) => {}

            Err(e) => return Err(e),
        };

        loop {
            println!("{}" , "Enter the name of the food you want to delete from the menu(enter (done) when you are finish entring item's name".bright_blue());

            let mut item_name = String::new();
            io::stdin().read_line(&mut item_name).unwrap();

            if item_name.trim() == "done" || item_name.trim() == "Done" {
                break;
            }

            let item_index = my_res
                .menu
                .iter()
                .position(|item| item.name == item_name.trim());

            let item_index = match item_index {
                Some(val) => val,

                None => {
                    return Err(ErrorType::NotFound(
                        "The item is not existed in this resturant's menu".to_string(),
                    ))
                }
            };

            my_res.menu.remove(item_index);
            println!(
                "{}",
                format!(
                    "The item named {} has been removed from the menu successfully",
                    item_name
                )
                .bright_green()
            );
        }

        let res_arr = match serde_json::to_string_pretty(&res_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_arr) {
            Ok(_) => return Ok("The menu has been changed successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn show_all_order(&self) -> Result<(), ErrorType> {
        let order_str = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let order_arr = if order_str.len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<Order>>(&order_str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        if order_arr.len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no orders for your resturant!".to_string(),
            ));
        }

        let wanted_arr = order_arr
            .iter()
            .filter(|order| order.resturant == self.resturant)
            .collect::<Vec<&Order>>();

        if wanted_arr.len() == 0 {
            println!(
                "{}",
                "There is no order for your resturant yet".bright_red()
            );
            return Ok(());
        }

        println!(
            "{}",
            format!("Here is all orders of {} ", self.resturant).bright_blue()
        );
        for order in wanted_arr {
            println!(
                "{}",
                format!("The customer's username : {}", order.username).bright_cyan()
            );
            println!("{}", format!("Items : {:#?}", order.items).bright_cyan());
            println!(
                "{}",
                format!("Total price : {}", order.total_price).bright_cyan()
            );
            println!(
                "{}",
                format!("Date time : {}", order.datetime).bright_cyan()
            );
            println!("{}", format!("Status : {}", order.status).bright_cyan());
            println!(
                "{}",
                "----------------------------------------".bright_white()
            );
        }
        Ok(())
    }

    fn show_all_order_datetime(&self) -> Result<(), ErrorType> {
        let order_str = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        if order_str.trim().len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no order for this resturant".to_string(),
            ));
        }

        let order_arr = match serde_json::from_str::<Vec<Order>>(&order_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let wanted_arr = order_arr
            .iter()
            .filter(|order| order.resturant == self.resturant)
            .collect::<Vec<&Order>>();

        if wanted_arr.len() == 0 {
            println!(
                "{}",
                "There is no order for your resturant yet".bright_red()
            );
            return Ok(());
        }

        println!(
            "{}",
            format!("Here is all orders' history of {} ", self.resturant).bright_blue()
        );
        for order in wanted_arr {
            println!(
                "{}",
                format!("The customer's username : {}", order.username).bright_cyan()
            );
            println!(
                "{}",
                format!("Date time : {}", order.datetime).bright_cyan()
            );
            println!("{}", format!("Status : {}", order.status).bright_cyan());
            println!(
                "{}",
                "----------------------------------------".bright_white()
            );
        }
        Ok(())
    }

    fn show_menu(&self) -> Result<(), ErrorType> {
        let rests_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut rests_arr = match serde_json::from_str::<Vec<Resturant>>(&rests_str) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let my_res = rests_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        println!("{}", "Here is your menu".bright_purple());
        let _ = match my_res.show_menu() {
            Ok(_) => return Ok(()),

            Err(e) => return Err(e),
        };
    }

    fn add_discount_to_resturant(&self) -> Result<String, ErrorType> {
        let rests_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut rests_arr = match serde_json::from_str::<Vec<Resturant>>(&rests_str) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let my_res = rests_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        println!("{}", "Please enter your discount percentage".bright_blue());

        let discount = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let result = match input.trim().parse::<u32>() {
                Ok(val) => val,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            };

            if result > 100 {
                println!("{}", "What ? more thank 100% off?!!!".bright_red());
                continue;
            } else {
                break result;
            }
        };

        my_res.discount = discount;

        let res_arr = match serde_json::to_string_pretty(&rests_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_arr) {
            Ok(_) => return Ok("The discount has been added successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_discount(&self) -> Result<String, ErrorType> {
        let rests_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut rests_arr = match serde_json::from_str::<Vec<Resturant>>(&rests_str) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let my_res = rests_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        if my_res.discount == 0 {
            return Err(ErrorType::NoDiscount(
                "Your resturnat has no disocunt to change!(First add a discount)".to_string(),
            ));
        }

        println!("{}", "Please enter your discount percentage".bright_blue());

        let discount = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let result = match input.trim().parse::<u32>() {
                Ok(val) => val,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            };

            if result > 100 {
                println!("{}", "What ? more thank 100% off?!!!".bright_red());
                continue;
            } else {
                break result;
            }
        };

        my_res.discount = discount;

        let res_arr = match serde_json::to_string_pretty(&rests_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_arr) {
            Ok(_) => return Ok("The discount has been changed successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn remove_discount(&self) -> Result<String, ErrorType> {
        let rests_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut rests_arr = match serde_json::from_str::<Vec<Resturant>>(&rests_str) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let my_res = rests_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        my_res.discount = 0;
        let res_arr = match serde_json::to_string_pretty(&rests_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_arr) {
            Ok(_) => return Ok("The discount has been removed successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_price(&self) -> Result<String, ErrorType> {
        let rests_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut rests_arr = match serde_json::from_str::<Vec<Resturant>>(&rests_str) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let my_res = rests_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        println!(
            "{}",
            "Enter the name of the item you wnat change its price".bright_purple()
        );
        let mut item_name = String::new();
        io::stdin().read_line(&mut item_name).unwrap();
        let item = my_res
            .menu
            .iter_mut()
            .find(|holder| holder.name == item_name.trim());

        let item = match item {
            Some(val) => val,

            None => {
                return Err(ErrorType::NotFound(
                    "This item is not existed on your menu".to_string(),
                ))
            }
        };

        println!("{}", "Enter your new price for this item".bright_purple());
        let price = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<f64>() {
                Ok(val) => break val,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            }
        };

        item.price = price;

        let res_arr = match serde_json::to_string_pretty(&rests_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_arr) {
            Ok(_) => return Ok("The item's price has been updated successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn add_quantity(&self) -> Result<String, ErrorType> {
        let rests_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut rests_arr = match serde_json::from_str::<Vec<Resturant>>(&rests_str) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let my_res = rests_arr.iter_mut().find(|res| res.name == self.resturant);

        let my_res = match my_res {
            Some(val) => val,

            None => return Err(ErrorType::NGH),
        };

        println!(
            "{}",
            "Enter the name of the item you wnat change its quantity".bright_purple()
        );

        let mut item_name = String::new();
        io::stdin().read_line(&mut item_name).unwrap();
        let item = my_res
            .menu
            .iter_mut()
            .find(|holder| holder.name == item_name.trim());

        let item = match item {
            Some(val) => val,

            None => {
                return Err(ErrorType::NotFound(
                    "This item is not existed on your menu".to_string(),
                ))
            }
        };

        println!(
            "{}",
            "Enter your the quantity you want to add to this item".bright_purple()
        );
        let quantity = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u32>() {
                Ok(val) => break val,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            }
        };

        item.quantity = quantity;

        let res_arr = match serde_json::to_string_pretty(&rests_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_arr) {
            Ok(_) => return Ok("The item's quantity has been updated successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_order_status(&self) -> Result<String, ErrorType> {
        let order_str = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut order_arr = if order_str.len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<Order>>(&order_str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        if order_arr.len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no orders for your resturant!".to_string(),
            ));
        }

        let mut wanted_arr = order_arr
            .iter_mut()
            .filter(|order| order.resturant == self.resturant)
            .collect::<Vec<&mut Order>>();

        if wanted_arr.len() == 0 {
            println!(
                "{}",
                "There is no order for your resturant yet".bright_red()
            );
        }

        let mut order_cnt = 1;

        for order in wanted_arr.iter() {
            println!("{}", format!("Name : {}", order.username).bright_cyan());
            println!("{}", format!("Items : {:#?}", order.items).bright_white());
            println!(
                "{}",
                format!("Date time : {}", order.datetime).bright_cyan()
            );
            println!("{}", format!("Status :{}", order.status).bright_cyan());
            println!("{}", format!("Order num : {}", order_cnt).bright_cyan());
            order_cnt += 1;
            println!(
                "{}",
                "----------------------------------------------------".bright_white()
            );
        }

        println!("{}", "Enter the number of the order".bright_blue());

        let order_num = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(val) => {
                    if val > wanted_arr.len() {
                        println!("{}", "Order number is out of bound!".blue());
                        continue;
                    }
                    break val;
                }

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                }
            }
        };

        println!("{}", "Enter the status".bright_purple());
        println!("1 : InProgress -- 2 : Canceled -- 3 : Delivered");

        let status = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let result = match input.trim().parse::<u32>() {
                Ok(val) => val,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            };

            match result {
                1 => break OrderStatus::Preparing,

                2 => break OrderStatus::Canceled,

                3 => break OrderStatus::Delivered,

                _ => {
                    println!("{}", "Invalid command".bright_red());
                    continue;
                }
            }
        };

        let users_str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        };

        let mut users_arr = match serde_json::from_str::<Vec<All>>(&users_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let filtered_arr = users_arr
            .iter_mut()
            .filter(|holder| {
                if let All::User(_) = holder {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&mut All>>();

        for user in filtered_arr {
            match user {
                All::User(user) => {
                    let result = user.order.iter_mut().enumerate().find(|(_, ord)| {
                        ord.datetime == wanted_arr[order_num - 1].datetime
                            && ord.resturant == wanted_arr[order_num - 1].resturant
                            && user.username == wanted_arr[order_num - 1].username
                    });

                    let index = match result {
                        Some(val) => val.0,
                        None => continue,
                    };
                    user.order[index].status = status.clone();
                    break;
                }

                _ => return Err(ErrorType::NGH),
            }
        }
        wanted_arr[order_num - 1].status = status;

        let order_arr = match serde_json::to_string_pretty(&order_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        let _ = match fs::write("data/order.json", order_arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        };

        let user_arr = match serde_json::to_string_pretty(&users_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/user.json", user_arr) {
            Ok(_) => return Ok("The item's status has been updated successfully".to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }
}
