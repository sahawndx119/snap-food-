//in the name of God//

use colored::Colorize;

use crate::admin::*;
use crate::handlres::*;
use crate::handlres_entry::Entry;
use crate::model::{Admin, ErrorType, ResturantOwner, User};
use std::process::Command;

use std::io;

pub trait CommandMenu {
    fn show_command_menu_and_handle(&mut self) -> Result<String, ErrorType>;
}

impl CommandMenu for User {
    fn show_command_menu_and_handle(&mut self) -> Result<String, ErrorType> {
        println!("{}", "Here is the command for you".bright_green());
        println!(
            "{}{}",
            "1 : View Resturant's".bright_purple(),
            "(Attention : From this command you can see the resturant's menu and order and also change status' order!!)"
                .bright_yellow()
        );
        println!("{}", "2 : View your order history".bright_purple());
        println!("{}", "3 : View resturants with discount".bright_purple());
        println!("{}", "4 : Change order status".bright_purple());
        println!("{}", "5 : Change username".bright_purple());
        println!("{}", "6 : Change password".bright_purple());

        println!(
            "{}{}",
            "Please enter your command".bright_purple(),
            "(Enter (done) when you are finish entering commands)".bright_yellow()
        );
        let command = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "done" || input.trim() == "Done" {
                return Ok("Done".to_string());
            }

            let res1 = match input.trim().parse::<u32>() {
                Ok(num) => num,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            };

            match res1 {
                1 => break 1,

                2 => break 2,

                3 => break 3,

                4 => break 4,

                5 => break 5,

                6 => break 6,

                _ => {
                    println!("{}", "Invalid command".bright_red());
                    continue;
                }
            }
        };

        clear_terminal();

        let _ = match command {
            1 => match self.show_resturants_and_order() {
                Ok(str) => {
                    if str == "back" {
                        return Err(ErrorType::BackOption);
                    }
                    return Ok(str);
                }

                Err(e) => return Err(e),
            },

            2 => match self.show_date_time() {
                Ok(_) => return Ok("Empty".to_string()),

                Err(e) => return Err(e),
            },

            3 => match self.show_resturants_with_discount() {
                Ok(_) => return Ok("Empty".to_string()),
                Err(e) => return Err(e),
            },

            4 => match self.change_order_staus() {
                Ok(str) => return Ok(str),

                Err(e) => return Err(e),
            },

            5 => match self.change_user_name() {
                Ok(val) => {
                    self.username = val;
                    return Ok("The username has been changed successfully".to_string());
                }

                Err(e) => return Err(e),
            },

            6 => match self.change_user_passwrod() {
                Ok(val) => {
                    self.password = val;
                    return Ok("The password has been changed successfully".to_string());
                }

                Err(e) => return Err(e),
            },

            _ => {
                return Err(ErrorType::NGH);
            }
        };
    }
}

impl CommandMenu for ResturantOwner {
    fn show_command_menu_and_handle(&mut self) -> Result<String, ErrorType> {
        println!("{}", "1 : Add items to the menu".bright_purple());
        println!("{}", "2 : Delete item from your menu".bright_purple());
        println!("{}", "3 : View orders of your resturant".bright_purple());
        println!(
            "{}",
            "4 : View your resturant's order history".bright_purple()
        );
        println!("{}", "5 : Show your resturant's menu".bright_purple());
        println!(
            "{}",
            "6 : Add discount for all of your items".bright_purple()
        );
        println!(
            "{}",
            "7 : Change discount for all of your items".bright_purple()
        );
        println!("{}", "8 : Remove the discount".bright_purple());
        println!("{}", "9 : Change the price".bright_purple());
        println!("{}", "10 : Add quantity".bright_purple());
        println!("{}", "11 : Change the status".bright_purple());
        println!("{}", "12 : Change username".bright_purple());
        println!("{}", "13 : Change password".bright_purple());
        println!("{}", "14 : Change resturant name".bright_purple());

        println!(
            "{}{}",
            "Please enter your command".bright_purple(),
            "(Enter (done) when you are finish entering commands)".bright_yellow()
        );
        let command = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "done" || input.trim() == "Done" {
                return Ok("Done".to_string());
            }

            let res1 = match input.trim().parse::<u32>() {
                Ok(num) => num,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            };

            clear_terminal();


            match res1 {
                1 => break 1,

                2 => break 2,

                3 => break 3,

                4 => break 4,

                5 => break 5,

                6 => break 6,

                7 => break 7,

                8 => break 8,

                9 => break 9,

                10 => break 10,

                11 => break 11,

                12 => break 12,

                13 => break 13,

                14 => break 14,

                _ => {
                    println!("{}", "Invalid command".bright_red());
                    continue;
                }
            }
        };

        match command {
            1 => match self.add_item_to_menu() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            2 => match self.delete_from_menu() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            3 => match self.show_all_order() {
                Ok(_) => return Ok("Empty".to_string()),

                Err(e) => return Err(e),
            },

            4 => match self.show_all_order_datetime() {
                Ok(_) => return Ok("Empty".to_string()),
                Err(e) => return Err(e),
            },

            5 => match self.show_menu() {
                Ok(_) => return Ok("Empty".to_string()),
                Err(e) => return Err(e),
            },

            6 => match self.add_discount_to_resturant() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            7 => match self.change_discount() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            8 => match self.remove_discount() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            9 => match self.change_price() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            10 => match self.add_quantity() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            11 => match self.change_order_status() {
                Ok(val) => return Ok(val),

                Err(e) => return Err(e),
            },

            12 => match self.change_user_name() {
                Ok(val) => {
                    self.owner = val;
                    return Ok("The username has been changed successfully".to_string());
                }

                Err(e) => return Err(e),
            },

            13 => match self.change_user_passwrod() {
                Ok(val) => {
                    self.password = val;
                    return Ok("The password has been changed successfully".to_string());
                }

                Err(e) => return Err(e),
            },

            14 => match self.change_resturant_name() {
                Ok(val) => {
                    self.resturant = val;
                    return Ok("Returant's name has been changed successfully".to_string());
                }

                Err(e) => return Err(e),
            },

            _ => {
                return Err(ErrorType::NGH);
            }
        }
    }
}

impl CommandMenu for Admin {
    fn show_command_menu_and_handle(&mut self) -> Result<String, ErrorType> {
        println!("{}", "1 : View all users(Every roles)".bright_purple());
        println!("{}", "2 : View all resturants".bright_purple());
        println!(
            "{}",
            "3 : View all orders(From all resturans)".bright_purple()
        );
        println!("{}", "4 : Ban Abusive users".bright_purple());
        println!(
            "{}",
            "5 : View all resturants with discount".bright_purple()
        );
        println!("{}", "6 : Change username".bright_purple());
        println!("{}", "7 : Change password".bright_purple());

        println!(
            "{}{}",
            "Please enter your command".bright_purple(),
            "(Enter (done) when you are finish entering commands)".bright_yellow()
        );
        let command = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "done" || input.trim() == "Done" {
                return Ok("Done".to_string());
            }

            let res1 = match input.trim().parse::<u32>() {
                Ok(num) => num,

                Err(_) => {
                    println!("{}", "Invalid number".bright_red());
                    continue;
                }
            };

            clear_terminal();

            match res1 {
                1 => break 1,

                2 => break 2,

                3 => break 3,

                4 => break 4,

                5 => break 5,

                6 => break 6,

                7 => break 7,

                _ => {
                    println!("{}", "Invalid command".bright_red());
                    continue;
                }
            }
        };
        match command {
            1 => match self.show_all_user() {
                Ok(_) => return Ok("Empty".to_string()),

                Err(e) => return Err(e),
            },

            2 => match self.show_all_resturant() {
                Ok(_) => return Ok("Empty".to_string()),
                Err(e) => return Err(e),
            },

            3 => match self.show_all_order() {
                Ok(_) => return Ok("Empty".to_string()),
                Err(e) => return Err(e),
            },

            4 => match self.ban_user() {
                Ok(str) => return Ok(str),
                Err(e) => return Err(e),
            },

            5 => match self.show_resturants_with_discount() {
                Ok(_) => return Ok("Empty".to_string()),

                Err(e) => return Err(e),
            },

            162 => match self.change_user_name() {
                Ok(val) => {
                    self.username = val;
                    return Ok("The username has been changed successfully".to_string());
                }

                Err(e) => return Err(e),
            },

            7 => match self.change_user_passwrod() {
                Ok(val) => {
                    self.password = val;
                    return Ok("The password has been changed successfully".to_string());
                }

                Err(e) => return Err(e),
            },

            _ => {
                return Err(ErrorType::NGH);
            }
        }
    }
}



fn clear_terminal() {
    let _ = Command::new("clear").status();
}