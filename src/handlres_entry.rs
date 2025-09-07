//in the name of God//

use crate::model::*;
use colored::Colorize;
use std::{fs, io};

pub trait Entry {
    fn login() -> Result<Self, ErrorType>
    where
        Self: Sized;

    fn get_login_info() -> Self;

    fn change_user_name(&self) -> Result<String, ErrorType>
    where
        Self: Sized;
    fn change_user_passwrod(&self) -> Result<String, ErrorType>
    where
        Self: Sized;

    fn signin() -> Result<Self, ErrorType>
    where
        Self: Sized;

    fn get_username_pass() -> (String, String) {
        println!("{}", "Please enter your username".bright_blue());
        println!("{}", "Username : ".bright_blue());
        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();

        println!("{}", "Please enter your username".bright_blue());
        println!("{}", "Password : ".bright_blue());
        let mut password = String::new();
        io::stdin().read_line(&mut password).unwrap();
        let username = username.trim();
        let password = password.trim();

        return (username.to_string(), password.to_string());
    }
}

impl Entry for User {
    fn get_login_info() -> Self {
        let (username, password) = User::get_username_pass();
        let output = Self {
            username: username.trim().to_string(),
            password: password.trim().to_string(),
            role: Role::User,
            order: Vec::new(),
        };

        output
    } //works successfully✅

    fn login() -> Result<Self, ErrorType> {
        let users = match fs::read_to_string("data/user.json") {
            Ok(value) => value,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ));
            }
        }; //checked✅

        if users.trim().len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no account existed".to_string(),
            ));
        }

        let mut user_input = User::get_login_info();

        let users = match serde_json::from_str::<Vec<All>>(&users) {
            Ok(value) => value,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let result = users.iter().find(|item| {
            if let All::User(user) = item {
                user.username == user_input.username && user_input.password == user.password
            } else {
                false
            }
        });

        match result {
            Some(val) => {
                if let All::User(item) = val {
                    user_input = item.clone();
                }
                return Ok(user_input);
            }

            None => {
                return Err(ErrorType::NotFound(
                    "Your user name or password is incorrect!".to_string(),
                ))
            }
        }
    } //works successfully✅

    fn signin() -> Result<Self, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        let (username, mut password) = User::get_username_pass();
        if password.trim().len() == 0 {
            return Err(ErrorType::Password(
                "You should enter a valid pass word".to_string(),
            ));
        }

        let result = arr.iter().find(|item| {
            if let All::User(value) = item {
                value.username == username
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let password = password_part(&mut password);

        let holder = User {
            username,
            password,
            role: Role::User,
            order: Vec::new(),
        };
        arr.push(All::User(holder.clone()));

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => return Ok(holder),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_user_name(&self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        println!("{}", "Please enter your new username".bright_blue());
        let mut new = String::new();
        io::stdin().read_line(&mut new).unwrap();

        let result = arr.iter().find(|item| {
            if let All::User(value) = item {
                value.username == new.trim()
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let user = arr.iter_mut().find(|user| {
            if let All::User(holder) = user {
                holder.username == self.username
            } else {
                false
            }
        });

        match user {
            Some(user) => match user {
                All::User(holder) => holder.username = new.trim().to_string(),

                _ => return Err(ErrorType::NGH),
            },

            _ => return Err(ErrorType::NGH),
        }

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }

        let order_str = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        };

        if order_str.trim().len() == 0 {
            return Ok(new.trim().to_string());
        }

        let mut order_arr = match serde_json::from_str::<Vec<Order>>(&order_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Cannot decode to json".to_string())),
        };

        order_arr
            .iter_mut()
            .filter(|order| order.username == self.username)
            .for_each(|order| order.username = new.trim().to_string());

        let order_str = match serde_json::to_string_pretty(&order_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/order.json", order_str) {
            Ok(_) => return Ok(new.trim().to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_user_passwrod(&self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        println!("{}", "Please enter your new password".bright_blue());
        let mut new = String::new();
        io::stdin().read_line(&mut new).unwrap();

        let user = arr.iter_mut().find(|user| {
            if let All::User(holder) = user {
                holder.username == self.username
            } else {
                false
            }
        });

        let new = password_part(&mut new);

        match user {
            Some(user) => match user {
                All::User(holder) => holder.password = new.trim().to_string(),

                _ => return Err(ErrorType::NGH),
            },

            _ => return Err(ErrorType::NGH),
        }

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => return Ok(new.trim().to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    } //works successfully✅
}

impl Entry for Admin {
    fn login() -> Result<Self, ErrorType> {
        let users = match fs::read_to_string("data/user.json") {
            Ok(value) => value,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ));
            }
        }; //checked✅

        let mut user_input = Admin::get_login_info();

        if users.trim().len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no account existed".to_string(),
            ));
        }

        let users: Vec<All> = match serde_json::from_str(&users) {
            Ok(value) => value,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let result = users.iter().find(|item| {
            if let All::Admin(user) = item {
                user.username == user_input.username && user_input.password == user.password
            } else {
                false
            }
        });

        match result {
            Some(val) => {
                if let All::Admin(item) = val {
                    user_input = item.clone();
                }

                return Ok(user_input);
            }

            None => {
                return Err(ErrorType::NotFound(
                    "Your user name or password is incorrect!".to_string(),
                ))
            }
        }
    } //works successfully✅

    fn get_login_info() -> Self {
        let (username, password) = Admin::get_username_pass();

        let output = Self {
            username: username.trim().to_string(),
            password: password.trim().to_string(),
            role: Role::User,
        };

        output
    }

    fn signin() -> Result<Self, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        let (username, mut password) = User::get_username_pass();

        let result = arr.iter().find(|item| {
            if let All::Admin(value) = item {
                value.username == username
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let password = password_part(&mut password);

        let holder = Admin {
            username,
            password,
            role: Role::Admin,
        };
        arr.push(All::Admin(holder.clone()));

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => return Ok(holder),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_user_name(&self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        println!("{}", "Please enter your new username".bright_blue());
        let mut new = String::new();
        io::stdin().read_line(&mut new).unwrap();

        let result = arr.iter().find(|item| {
            if let All::Admin(value) = item {
                value.username == new.trim()
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let user = arr.iter_mut().find(|user| {
            if let All::Admin(holder) = user {
                holder.username == self.username
            } else {
                false
            }
        });

        match user {
            Some(user) => match user {
                All::Admin(holder) => holder.username = new.trim().to_string(),

                _ => return Err(ErrorType::NGH),
            },

            _ => return Err(ErrorType::NGH),
        }

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => return Ok(new.trim().to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_user_passwrod(&self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        println!("{}", "Please enter your new password".bright_blue());
        let mut new = String::new();
        io::stdin().read_line(&mut new).unwrap();

        let user = arr.iter_mut().find(|user| {
            if let All::User(holder) = user {
                holder.username == self.username
            } else {
                false
            }
        });

        let new = password_part(&mut new);

        match user {
            Some(user) => match user {
                All::User(holder) => holder.password = new.trim().to_string(),

                _ => return Err(ErrorType::NGH),
            },

            _ => return Err(ErrorType::NGH),
        }

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => return Ok(new.trim().to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    } //works successfully✅
}

impl Entry for ResturantOwner {
    fn login() -> Result<Self, ErrorType> {
        let users = match fs::read_to_string("data/user.json") {
            Ok(value) => value,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ));
            }
        }; //checked✅

        let mut user_input = ResturantOwner::get_login_info();

        if users.trim().len() == 0 {
            return Err(ErrorType::EmptyFile(
                "There is still no account existed".to_string(),
            ));
        }

        let users: Vec<All> = match serde_json::from_str(&users) {
            Ok(value) => value,

            Err(_) => return Err(ErrorType::JsonDecode("Can not decode the json".to_string())),
        };

        let result = users.iter().find(|item| {
            if let All::ResturantOwner(user) = item {
                user.owner == user_input.owner
                    && user_input.password == user.password
                    && user_input.resturant == user.resturant
            } else {
                false
            }
        });

        match result {
            Some(val) => {
                if let All::ResturantOwner(item) = val {
                    user_input = item.clone();
                }
                return Ok(user_input);
            }

            None => {
                return Err(ErrorType::NotFound(
                    "Your user name or password is incorrect!".to_string(),
                ))
            }
        }
    } //works successfully✅

    fn get_login_info() -> Self {
        let (username, password) = ResturantOwner::get_username_pass();

        println!("{}", "Please enter your resturant's name".bright_blue());
        println!("{}", "Resturant's name".bright_blue());
        let mut r_name = String::new();
        io::stdin().read_line(&mut r_name).unwrap();

        let output = Self {
            owner: username.trim().to_string(),
            password: password.trim().to_string(),
            role: Role::User,
            resturant: r_name.trim().to_string(),
        };

        output
    }

    fn signin() -> Result<Self, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        let (username, mut password) = User::get_username_pass();

        let result = arr.iter().find(|item| {
            if let All::ResturantOwner(value) = item {
                value.owner == username
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let password = password_part(&mut password);

        println!("{}", "Please enter your resturant's name".bright_blue());
        println!("{}", "Resturant's name : ".bright_blue());
        let mut r_name = String::new();
        io::stdin().read_line(&mut r_name).unwrap();

        let result = arr.iter().find(|item| {
            if let All::ResturantOwner(value) = item {
                value.resturant == r_name.trim()
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let holder1 = ResturantOwner {
            owner: username.clone(),
            password,
            role: Role::ResturantOwner,
            resturant: r_name.trim().to_string(),
        };
        arr.push(All::ResturantOwner(holder1.clone()));

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };

        println!(
            "{}",
            "Please enter the category of your resturant(With numbers)".bright_blue()
        );
        println!(
            "{}",
            "1 : Fast Food -- 2 : Traditional Food -- 3 : Dessert\n4 : Persian -- 5 : Sea Food -- 6 : Vegan".bright_yellow()
        );

        let category = loop {
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

        let category = loop {
            match category {
                1 => break Category::FastFood,

                2 => break Category::TraditionalFood,

                3 => break Category::Dessert,

                4 => break Category::Persian,

                5 => break Category::SeaFood,

                6 => break Category::Vegan,

                _ => {
                    println!("The command is invalid");
                    continue;
                }
            }
        };

        let str2 = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr2;
        if str2.len() == 0 {
            arr2 = Vec::new();
        } else {
            arr2 = match serde_json::from_str::<Vec<Resturant>>(&str2) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        let holder = Resturant {
            name: r_name.trim().to_string(),
            owner: username.trim().to_string(),
            category: category,
            menu: Vec::new(),
            discount: 0,
        };

        arr2.push(holder);

        let arr2 = match serde_json::to_string_pretty(&arr2) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };

        println!("{}", "Do you want to make your menu now?".bright_yellow());
        let answer = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "Yes" || input == "yes" {
                break true;
            } else if input == "No" || input == "no" {
                break false;
            } else {
                println!("{}", "Invalid!(Enter (Yes) or (No))".bright_red());
                continue;
            }
        };

        let _ = match fs::write("data/resturant.json", arr2) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moments".to_string(),
                ))
            }
        };

        let _ = match fs::write("data/user.json", arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moments".to_string(),
                ))
            }
        };

        if answer {
            let str2 = match fs::read_to_string("data/resturant.json") {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::ReadFromFile(
                        "Cannot read from the file".to_string(),
                    ))
                }
            }; //checked✅

            let mut arr2;
            if str2.len() == 0 {
                arr2 = Vec::new();
            } else {
                arr2 = match serde_json::from_str::<Vec<Resturant>>(&str2) {
                    Ok(val) => val,

                    Err(_) => {
                        return Err(ErrorType::JsonDecode(
                            "Cannot decode to json format".to_string(),
                        ))
                    }
                };
            }
            match ResturantOwner::make_menu(username.trim(), r_name.trim(), &mut arr2) {
                Ok(_) => {}

                Err(e) => {
                    println!("{}", format!("{}", e).bright_red());
                    println!(
                        "{}",
                        "The menu did not added you can modify it later!".bright_red()
                    );
                }
            }

            let arr2 = match serde_json::to_string_pretty(&arr2) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonEncode(
                        "Cannot encode to json format!".to_string(),
                    ))
                }
            };

            let _ = match fs::write("data/resturant.json", arr2) {
                Ok(_) => {}

                Err(_) => {
                    return Err(ErrorType::WriteOnFile(
                        "Cannot write on this file at this moments".to_string(),
                    ))
                }
            };
        }

        return Ok(holder1);
    }

    fn change_user_name(&self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        println!("{}", "Please enter your new user name".bright_blue());
        let mut new = String::new();
        io::stdin().read_line(&mut new).unwrap();

        let result = arr.iter().find(|item| {
            if let All::ResturantOwner(value) = item {
                value.owner == new.trim()
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let user = arr.iter_mut().find(|user| {
            if let All::ResturantOwner(holder) = user {
                holder.owner == self.owner
            } else {
                false
            }
        });

        match user {
            Some(user) => match user {
                All::ResturantOwner(holder) => holder.owner = new.trim().to_string(),

                _ => return Err(ErrorType::NGH),
            },

            _ => return Err(ErrorType::NGH),
        }

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }

        let res_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        };

        if res_str.trim().len() == 0 {
            return Ok(new.trim().to_string());
        }

        let mut res_arr = match serde_json::from_str::<Vec<Resturant>>(&res_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Cannot decode to json".to_string())),
        };

        res_arr
            .iter_mut()
            .filter(|res| res.owner == self.owner)
            .for_each(|res| res.owner = new.trim().to_string());

        let res_str = match serde_json::to_string_pretty(&res_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_str) {
            Ok(_) => return Ok(new.trim().to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }

    fn change_user_passwrod(&self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        println!("{}", "Please enter your new password".bright_blue());
        let mut new = String::new();
        io::stdin().read_line(&mut new).unwrap();

        let user = arr.iter_mut().find(|user| {
            if let All::User(holder) = user {
                holder.username == self.owner
            } else {
                false
            }
        });

        let new = password_part(&mut new);

        match user {
            Some(user) => match user {
                All::User(holder) => holder.password = new.trim().to_string(),

                _ => return Err(ErrorType::NGH),
            },

            _ => return Err(ErrorType::NGH),
        }

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => return Ok(new.trim().to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    } //works successfully✅
}

impl ResturantOwner {
    fn make_menu(
        username: &str,
        res_name: &str,
        resturants: &mut Vec<Resturant>,
    ) -> Result<(), ErrorType> {
        let result = {
            let res = resturants
                .iter_mut()
                .find(|res| (res.owner == username.trim()) && res.name == res_name.trim());

            match res {
                Some(val) => val,
                None => {
                    return Err(ErrorType::NotFound(
                        "Cannot find the item on your purchase list".to_string(),
                    ))
                }
            }
        };

        loop {
            let holder = match ResturantOwner::make_item() {
                Some(val) => val,

                None => break,
            };

            println!(
                "{}",
                format!(
                    "The item named {} added to the menu successfully",
                    holder.name
                )
                .bright_green()
            );
            result.menu.push(holder);
        }

        Ok(())
    }

    fn make_item() -> Option<Item> {
        println!(
            "{}",
            "Please enter the name of the item(enter (Done) if you are finish entering item)"
                .bright_blue()
        );
        println!("{}", "Name : ".bright_blue());
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();

        if name.trim() == "done" || name.trim() == "Done" {
            return None;
        }

        println!("{}", "Please enter the price of the item".bright_blue());
        let price = loop {
            let mut price_input = String::new();
            io::stdin().read_line(&mut price_input).unwrap();

            let holder = match price_input.trim().parse::<f64>() {
                Ok(num) => num,

                Err(_) => continue,
            };
            break holder;
        };

        println!("{}", "Please enter the quantity of the item".bright_blue());
        let quantity = loop {
            let mut quantity_input = String::new();
            io::stdin().read_line(&mut quantity_input).unwrap();

            let holder = match quantity_input.trim().parse::<u32>() {
                Ok(num) => num,

                Err(_) => continue,
            };
            break holder;
        };

        let holder = Item {
            name: name.trim().to_string(),

            price: price,

            quantity,
        };
        Some(holder)
    }

    pub fn change_resturant_name(&self) -> Result<String, ErrorType> {
        let str = match fs::read_to_string("data/user.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        }; //checked✅

        let mut arr;
        if str.len() == 0 {
            arr = Vec::new();
        } else {
            arr = match serde_json::from_str::<Vec<All>>(&str) {
                Ok(val) => val,

                Err(_) => {
                    return Err(ErrorType::JsonDecode(
                        "Cannot decode to json format".to_string(),
                    ))
                }
            };
        }

        println!("{}", "Please enter your new resturant's name".bright_blue());
        let mut new = String::new();
        io::stdin().read_line(&mut new).unwrap();

        let result = arr.iter().find(|item| {
            if let All::ResturantOwner(value) = item {
                value.resturant == new.trim()
            } else {
                false
            }
        });

        match result {
            Some(_) => {
                return Err(ErrorType::ReapetedName(
                    "An account with this role already exists under a different name.".to_string(),
                ))
            }

            None => {}
        }

        let user = arr.iter_mut().find(|user| {
            if let All::ResturantOwner(holder) = user {
                holder.owner == self.owner
            } else {
                false
            }
        });

        match user {
            Some(user) => match user {
                All::ResturantOwner(holder) => holder.resturant = new.trim().to_string(),

                _ => return Err(ErrorType::NGH),
            },

            _ => return Err(ErrorType::NGH),
        }

        let arr = match serde_json::to_string_pretty(&arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/user.json", arr) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }

        let res_str = match fs::read_to_string("data/resturant.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        };

        if res_str.trim().len() == 0 {
            return Ok(new.trim().to_string());
        }

        let mut res_arr = match serde_json::from_str::<Vec<Resturant>>(&res_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Cannot decode to json".to_string())),
        };

        res_arr
            .iter_mut()
            .filter(|res| res.owner == self.owner)
            .for_each(|res| res.name = new.trim().to_string());

        let res_str = match serde_json::to_string_pretty(&res_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };

        match fs::write("data/resturant.json", res_str) {
            Ok(_) => {}

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }

        let order_str = match fs::read_to_string("data/order.json") {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::ReadFromFile(
                    "Cannot read from the file".to_string(),
                ))
            }
        };

        if order_str.trim().len() == 0 {
            return Ok(new.trim().to_string());
        }

        let mut order_arr = match serde_json::from_str::<Vec<Order>>(&order_str) {
            Ok(val) => val,

            Err(_) => return Err(ErrorType::JsonDecode("Cannot decode to json".to_string())),
        };

        order_arr
            .iter_mut()
            .filter(|order| order.resturant == self.resturant)
            .for_each(|order| order.resturant = new.trim().to_string());

        let order_str = match serde_json::to_string_pretty(&order_arr) {
            Ok(val) => val,

            Err(_) => {
                return Err(ErrorType::JsonEncode(
                    "Cannot encode to json format!".to_string(),
                ))
            }
        };
        match fs::write("data/order.json", order_str) {
            Ok(_) => return Ok(new.trim().to_string()),

            Err(_) => {
                return Err(ErrorType::WriteOnFile(
                    "Cannot write on this file at this moment".to_string(),
                ))
            }
        }
    }
}

pub fn password_security_check(pass: &mut String) -> Security {
    let mut flag_cnt = 0;
    let mut upper_letter_flag = false;
    let mut lower_letter_flag = false;
    let mut number_flag = false;
    for letter in pass.bytes() {
        if !upper_letter_flag && (letter >= 65 && letter <= 90) {
            upper_letter_flag = true;
            flag_cnt += 1;
        }
        if !lower_letter_flag && (letter >= 97 && letter <= 122) {
            lower_letter_flag = true;
            flag_cnt += 1;
        }
        if !number_flag && (letter >= 48 && letter <= 57) {
            number_flag = true;
            flag_cnt += 1;
        }
    }
    let special_chars: [char; 25] = [
        '!', '@', '#', '$', '%', '&', '*', '(', ')', '_', '+', '-', '=', '[', ']', '{', '}', ';',
        ':', '\'', '"', ',', '.', '?', '/',
    ];
    for letter in special_chars {
        if pass.contains(letter) {
            flag_cnt += 1;
            break;
        }
    }

    let holder = match flag_cnt {
        1 => Security::Weak,
        2 => Security::Good,
        3 => Security::Powerful,
        4 => Security::AllSecure,

        _ => Security::AllSecure,
    };

    holder
}

pub fn password_part(password: &mut String) -> String {
    loop {
        match password_security_check(password) {
            Security::Weak => {
                println!("{}", format!("{}", Security::Weak).bright_white());
                println!("{}" , "!!ATTENTION!!If you want a stronger password, try using a mix of lowercase and uppercase letters, numbers, and some symbols 'like @, _ , or ! and ...'".bright_yellow());
                println!("{}", "Do you want to change your password?".bright_blue());
                let answer = loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();
                    if input == "Yes" || input == "yes" {
                        break true;
                    } else if input == "No" || input == "no" {
                        break false;
                    } else {
                        println!("{}", "Invalid!(Enter (Yes) or (No))".bright_red());
                        continue;
                    }
                };
                if answer {
                    println!("{}", "Great! So enter your new pass word!".bright_blue());
                    password.clear();
                    io::stdin().read_line(password).unwrap();
                    continue;
                } else {
                    println!(
                        "{}",
                        "Ok but your password is not all secure!!".bright_yellow()
                    );
                    break;
                }
            }
            Security::Good => {
                println!("{}", Security::Good);
                println!("{}" , "!!ATTENTION!!If you want a stronger password, try using a mix of lowercase and uppercase letters, numbers, and some symbols 'like @, _ , or ! and ...'".bright_yellow());
                println!("{}", "Do you want to change your password?".bright_blue());
                let answer = loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();
                    if input == "Yes" || input == "yes" {
                        break true;
                    } else if input == "No" || input == "no" {
                        break false;
                    } else {
                        println!("{}", "Invalid!(Enter (Yes) or (No))".bright_red());
                        continue;
                    }
                };
                if answer {
                    println!("{}", "Great! So enter your new password!".bright_blue());
                    password.clear();
                    io::stdin().read_line(password).unwrap();
                    continue;
                } else {
                    println!(
                        "{}",
                        "Ok but your password is not all secure!!".bright_yellow()
                    );
                    break;
                }
            }
            Security::Powerful => {
                println!("{}", Security::Powerful);
                println!("{}" , "!!ATTENTION!!If you want a stronger password, try using a mix of lowercase and uppercase letters, numbers, and some symbols 'like @, _ , or ! and ...'".bright_yellow());
                println!("{}", "Do you want to change your password?".bright_blue());
                let answer = loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();
                    if input == "Yes" || input == "yes" {
                        break true;
                    } else if input == "No" || input == "no" {
                        break false;
                    } else {
                        println!("{}", "Invalid!(Enter (Yes) or (No))".bright_red());
                        continue;
                    }
                };
                if answer {
                    println!("{}", "Great! So enter your new password!".bright_blue());
                    password.clear();
                    io::stdin().read_line(password).unwrap();
                    continue;
                } else {
                    println!(
                        "{}",
                        "Ok but your password is not all secure!!".bright_yellow()
                    );
                    break;
                }
            }
            Security::AllSecure => {
                println!("{}", Security::AllSecure);
                break;
            }
        }
    }
    loop {
        println!("{}", "Confirm your password".bright_blue());

        println!("Congirm password :");

        let mut conf_pass = String::new();
        io::stdin().read_line(&mut conf_pass).unwrap();

        if conf_pass.trim() == password.trim() {
            return format!("{}", password.trim());
        } else {
            println!(
                "{}",
                "The confirmation password doesn’t match. Please double-check it.".bright_red()
            );
            continue;
        }
    }
}
