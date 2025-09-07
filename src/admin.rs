//in the name of God//

use crate::{
    handlres::{ROCapabilities, UserCapabilities},
    model::{Admin, All, ErrorType, Order, Resturant, Role, User},
};

use std::{fs, io};

use colored::Colorize;

pub trait AdminCapabilities {
    fn show_all_user(&self) -> Result<(), ErrorType>;
    fn show_all_resturant(&self) -> Result<(), ErrorType>;
    fn show_all_order(&self) -> Result<(), ErrorType>;
    fn ban_user(&self) -> Result<String, ErrorType>;
    fn show_resturants_with_discount(&self) -> Result<(), ErrorType>;
}

impl AdminCapabilities for Admin {
    fn show_all_user(&self) -> Result<(), ErrorType> {
        let user_str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let user_arr = match serde_json::from_str::<Vec<All>>(&user_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        for user in &user_arr {
            match user {
                All::User(user) => {
                    println!(
                        "{}",
                        format!("Username : {}", user.username).bright_purple()
                    );
                    println!("{}", "Role : User".bright_cyan());
                    for order in user.order.iter() {
                        println!(
                            "{}",
                            format!("Resturant's name : {}", order.resturant).bright_purple()
                        );
                        println!("{}", format!("Items : {:#?}", order.items).bright_white());
                        println!(
                            "{}",
                            format!("Datetime : {} ", order.datetime).bright_purple()
                        );
                        println!("{}", format!("Status : {}", order.status).bright_purple());
                        println!("{}", "----------------------------------".bright_white());
                    }
                    if user.order.len() == 0 {
                        println!("{}", "----------------------------------".bright_white());
                    }
                }

                All::Admin(admin) => {
                    println!(
                        "{}",
                        format!("Username : {}", admin.username).bright_purple()
                    );
                    println!("{}", "Role : Admin".bright_cyan());
                    println!("{}", "----------------------------------".bright_white());
                }

                All::ResturantOwner(ro) => {
                    println!("{}", format!("Owner : {}", ro.owner).bright_purple());
                    println!(
                        "{}",
                        format!("Resturant : {}", ro.resturant).bright_purple()
                    );
                    println!("{}", "Role : ResturantOwner".bright_cyan());
                    println!("{}", "----------------------------------".bright_white());
                }
            }
        }
        Ok(())
    }

    fn show_all_resturant(&self) -> Result<(), ErrorType> {
        let res_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let res_arr = if res_str.len() == 0 {
            Vec::new()
        } else {
            match serde_json::from_str::<Vec<Resturant>>(&res_str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        if res_arr.len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no resturant available".to_string(),
            ));
        }

        for res in &res_arr {
            println!("{}", format!("Name : {}", res.name).bright_purple());
            println!("{}", format!("Owner : {}", res.owner).bright_purple());
            println!("{}", format!("Category : {}", res.category).bright_cyan());
            println!("{}", "Menu".bright_green());
            match res.show_menu() {
                Ok(_) => {}

                Err(e) => {
                    if let ErrorType::EmptyFile(val) = e {
                        println!("{}", val.bright_red());
                    }
                }
            }
        }
        Ok(())
    }

    fn show_all_order(&self) -> Result<(), ErrorType> {
        let order_str = match fs::read_to_string("data/resturant.json") {
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
            match serde_json::from_str::<Vec<Resturant>>(&order_str) {
                Ok(val) => val,

                Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
            }
        };

        if order_arr.len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no order on our list!!".to_string(),
            ));
        }
        let users_str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let users_arr = match serde_json::from_str::<Vec<All>>(&users_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let users_arr = users_arr
            .iter()
            .filter(|item| {
                if let All::ResturantOwner(_) = item {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&All>>();

        println!(
            "{}",
            "Here is all orders from all resturants".on_bright_green()
        );
        for res in users_arr {
            let holder = match res {
                All::ResturantOwner(val) => val,

                _ => return Err(ErrorType::NGH),
            };

            match holder.show_all_order() {
                Ok(_) => {}

                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn ban_user(&self) -> Result<String, ErrorType> {
        let all_users_str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut all_user = match serde_json::from_str::<Vec<All>>(&all_users_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        println!(
            "{}",
            "Please enter the role of the user you want to ban".bright_purple()
        );
        println!(
            "{}",
            "Role : 1 : User -- 2 : Admin -- 3 : ResturantOwner".bright_yellow()
        );
        let role = loop {
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

        let last_message = match role {
            1 => {
                println!(
                    "{}",
                    "Please enter the username of user you want to ban".bright_blue()
                );
                println!("{}", "Username : ".bright_blue());
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();

                let res_of_search = all_user.iter_mut().enumerate().find(|(_, item)| {
                    if let All::User(user) = item {
                        user.username == username.trim()
                    } else {
                        false
                    }
                });

                let (user_index, _) =
                    match res_of_search {
                        Some((index, val)) => (index, val),

                        None => return Err(ErrorType::NotFound(
                            "The user is not existed . You can read the usernames and try again"
                                .to_string(),
                        )),
                    };

                all_user.remove(user_index);

                let orders_str = match fs::read_to_string("data/order.json") {
                    Ok(val) => val,

                    Err(_) => {
                        return Err(ErrorType::ReadFromFile(
                            "Cannot read from the file".to_string(),
                        ))
                    }
                }; //checked✅

                let mut orders_arr = if orders_str.trim().len() == 0 {
                    Vec::new()
                } else {
                    match serde_json::from_str::<Vec<Order>>(&orders_str) {
                        Ok(val) => val,

                        Err(_) => {
                            return Err(ErrorType::JsonDecode(
                                "Can not decode the json".to_string(),
                            ))
                        }
                    }
                };

                let index_to_remove_cnt = orders_arr
                    .iter()
                    .enumerate()
                    .filter(|(_, order)| order.username == username.trim())
                    .count();

                for _ in 0..index_to_remove_cnt {
                    let result = orders_arr
                        .iter()
                        .enumerate()
                        .find(|(_, order)| order.username == username.trim())
                        .map(|(index, _)| index);

                    let _ = match result {
                        Some(index) => orders_arr.remove(index),

                        None => break,
                    };
                }

                let orders_json = match serde_json::to_string_pretty(&orders_arr) {
                    Ok(val) => val,

                    Err(_) => {
                        return Err(ErrorType::JsonEncode(
                            "Cannot encode to json format!".to_string(),
                        ))
                    }
                };

                let _ = match fs::write("data/order.json", orders_json) {
                    Ok(_) => {}

                    Err(_) => {
                        return Err(ErrorType::WriteOnFile(
                            "Cannot write on this file at this moment".to_string(),
                        ))
                    }
                };
                "The abusive user has been banned successfully".to_string()
            }

            2 => {
                println!(
                    "{}",
                    "Please enter the username of admin you want to ban".bright_blue()
                );
                println!("{}", "Username : ".bright_blue());
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();

                let res_of_search = all_user.iter_mut().enumerate().find(|(_, item)| {
                    if let All::Admin(user) = item {
                        user.username == username.trim()
                    } else {
                        false
                    }
                });

                let (user_index, _) =
                    match res_of_search {
                        Some((index, val)) => (index, val),

                        None => return Err(ErrorType::NotFound(
                            "The Admin is not existed . You can read the usernames and try again"
                                .to_string(),
                        )),
                    };

                all_user.remove(user_index);

                "The abusive Admin has been banned successfully".to_string()
            }

            3 => {
                println!(
                    "{}",
                    "Please enter the Onwer of the resturant you want to ban".bright_blue()
                );
                println!("{}", "Username : ".bright_blue());
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();

                println!(
                    "{}",
                    "Please enter the name of this users's returant".bright_blue()
                );
                let mut res_name = String::new();
                io::stdin().read_line(&mut res_name).unwrap();

                let res_of_search = all_user.iter_mut().enumerate().find(|(_, item)| {
                    if let All::ResturantOwner(user) = item {
                        user.owner == username.trim() && user.resturant == res_name.trim()
                    } else {
                        false
                    }
                });

                let (user_index, _) =
                    match res_of_search {
                        Some((index, val)) => (index, val),

                        None => return Err(ErrorType::NotFound(
                            "The user is not existed . You can read the usernames and try again"
                                .to_string(),
                        )),
                    };

                all_user.remove(user_index);

                let rests_str = match fs::read_to_string("data/resturant.json") {
                    Ok(val) => val,

                    Err(_) => {
                        return Err(ErrorType::ReadFromFile(
                            "Cannot read from the file".to_string(),
                        ))
                    }
                }; //checked✅

                let mut rests_arr = if rests_str.trim().len() == 0 {
                    Vec::new()
                } else {
                    match serde_json::from_str::<Vec<Resturant>>(&rests_str) {
                        Ok(val) => val,
                        Err(_) => {
                            return Err(ErrorType::JsonDecode(
                                "Can not decode the json".to_string(),
                            ))
                        }
                    }
                };

                let result = rests_arr
                    .iter()
                    .enumerate()
                    .find(|(_, res)| res.name == res_name.trim());

                let (res_index, _) = match result {
                    Some(val) => val,

                    None => {
                        return Err(ErrorType::NotFound(
                            "Cannot find this resturant on resturants' list ".to_string(),
                        ))
                    }
                };

                rests_arr.remove(res_index);

                let rests_json = match serde_json::to_string_pretty(&rests_arr) {
                    Ok(val) => val,
                    Err(_) => {
                        return Err(ErrorType::JsonEncode(
                            "Cannot encode to json format!".to_string(),
                        ))
                    }
                };

                let _ = match fs::write("data/resturant.json", rests_json) {
                    Ok(_) => {}

                    Err(_) => {
                        return Err(ErrorType::WriteOnFile(
                            "Cannot write on this file at this moment".to_string(),
                        ))
                    }
                };

                let orders_str = match fs::read_to_string("data/order.json") {
                    Ok(val) => val,

                    Err(_) => {
                        return Err(ErrorType::ReadFromFile(
                            "Cannot read from the file".to_string(),
                        ))
                    }
                };

                let mut orders_arr = match serde_json::from_str::<Vec<Order>>(&orders_str) {
                    Ok(val) => val,

                    Err(_) => {
                        return Err(ErrorType::JsonDecode("Can not decode the json".to_string()))
                    }
                };

                let index_to_remove_cnt = orders_arr
                    .iter()
                    .enumerate()
                    .filter(|(_, order)| order.resturant == res_name.trim())
                    .count();

                for _ in 0..index_to_remove_cnt {
                    let result = orders_arr
                        .iter()
                        .enumerate()
                        .find(|(_, order)| order.resturant == res_name.trim())
                        .map(|(index, _)| index);

                    let _ = match result {
                        Some(index) => orders_arr.remove(index),

                        None => break,
                    };
                }

                let orders_json = match serde_json::to_string_pretty(&orders_arr) {
                    Ok(val) => val,

                    Err(_) => {
                        return Err(ErrorType::JsonEncode(
                            "Cannot encode to json format!".to_string(),
                        ))
                    }
                };

                let _ = match fs::write("data/order.json", orders_json) {
                    Ok(_) => {}

                    Err(_) => {
                        return Err(ErrorType::WriteOnFile(
                            "Cannot write on this file at this moment".to_string(),
                        ))
                    }
                };

                let users = all_user
                    .iter_mut()
                    .filter(|item| if let All::User(_) = item { true } else { false })
                    .collect::<Vec<&mut All>>();

                for user in users {
                    match user {
                        All::User(user) => loop {
                            let result = user
                                .order
                                .iter()
                                .enumerate()
                                .find(|(_, order)| order.resturant == res_name.trim())
                                .map(|(index, _)| index);

                            let _ = match result {
                                Some(index) => user.order.remove(index),

                                None => break,
                            };
                        },

                        _ => return Err(ErrorType::NGH),
                    }
                }

                "The abusive user has been banned successfully".to_string()
            }

            _ => {
                return Err(ErrorType::InvalidCommand(
                    "This command is not valid!".to_string(),
                ))
            }
        };

        let user_arr = match serde_json::to_string_pretty(&all_user) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode it to json".to_string(),
                ))
            }
        };

        match fs::write("data/user.json", user_arr) {
            Ok(_) => return Ok(last_message),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        };
    }

    fn show_resturants_with_discount(&self) -> Result<(), ErrorType> {
        let holder = User {
            username: "".to_string(),
            password: "".to_string(),
            role: Role::User,
            order: Vec::new(),
        };

        match holder.show_resturants_with_discount() {
            Ok(_) => return Ok(()),

            Err(e) => return Err(e),
        }
    }
}
