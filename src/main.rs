//in the name of God//

use std::io;

use mini_projects3::{
    handlres_entry::Entry,
    menu_command_help::*,
    model::{Admin, All, ResturantOwner, User},
};

use colored::Colorize;


fn main() {
    println!("{}", "Hello welcome to snapp food!".bright_green());
    println!("{}", "Please enter the command".bright_blue());
    println!(
        "{}",
        "1 : Sign up -- 2 : Already have an account?(so login :) )".bright_blue()
    );
    let command = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let holder = match input.trim().parse::<u32>() {
            Ok(val) => val,

            Err(_) => {
                println!("{}", "Invalid number".bright_red());
                continue;
            }
        };

        match holder {
            1 => break holder,

            2 => break holder,

            _ => {
                println!("{}", "This command is not valid".bright_red());
                continue;
            }
        }
    };

    let mut command_1_cnt = 0;
    let mut command_2_cnt = 0;

    let role_of_the_user = loop {
        match command {
            1 => {
                if command_1_cnt == 0 {
                    println!(
                        "{}",
                        "Great !! so now tell us what will be your role on our application?"
                            .bright_green()
                    );
                }
                command_1_cnt += 1;
                println!(
                    "{}",
                    "1 : User -- 2 : ResturantOwner -- 3 : Admin".bright_yellow()
                );
                let role = loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    let num = match input.trim().parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("{}", "Invalid number".bright_red());
                            continue;
                        }
                    };
                    match num {
                        1 => break 1,

                        2 => break 2,

                        3 => break 3,

                        _ => {
                            println!("{}", "The command is not valid".bright_red());
                            continue;
                        }
                    }
                };
                let role = match role {
                    1 => {
                        let holder = match User::signin() {
                            Ok(val) => {
                                println!(
                                    "{}",
                                    "Your account has been successfully created.Welcome Abroad!"
                                        .on_bright_green()
                                );
                                val
                            }

                            Err(e) => {
                                println!("{}", format!("{e}").bright_red());
                                continue;
                            }
                        };

                        All::User(holder)
                    }

                    2 => {
                        let holder = match ResturantOwner::signin() {
                            Ok(val) => {
                                println!(
                                    "{}",
                                    "Your account has been successfully created.Welcome Abroad!"
                                        .on_bright_green()
                                );
                                val
                            }

                            Err(e) => {
                                println!("{}", format!("{e}").bright_red());
                                continue;
                            }
                        };

                        All::ResturantOwner(holder)
                    }

                    3 => {
                        let holder = match Admin::signin() {
                            Ok(val) => {
                                println!(
                                    "{}",
                                    "Your account has been successfully created.Welcome Abroad!"
                                        .on_bright_green()
                                );
                                val
                            }

                            Err(e) => {
                                println!("{}", format!("{e}").bright_red());
                                continue;
                            }
                        };

                        All::Admin(holder)
                    }

                    _ => {
                        continue;
                    }
                };
                break role;
            }

            2 => {
                if command_2_cnt == 0 {
                    println!(
                        "{}",
                        "Great !! so now tell us what is your role on our application?"
                            .bright_green()
                    );
                }
                command_2_cnt += 1;
                println!(
                    "{}",
                    "1 : User -- 2 : ResturantOwner -- 3 : Admin".bright_yellow()
                );
                let role = loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();

                    let num = match input.trim().parse::<u32>() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("{}", "Invalid number".bright_red());
                            continue;
                        }
                    };
                    match num {
                        1 => break 1,

                        2 => break 2,

                        3 => break 3,

                        _ => {
                            println!("{}", "The command is not valid".bright_red());
                            continue;
                        }
                    }
                };
                let role = match role {
                    1 => {
                        let holder = match User::login() {
                            Ok(val) => {
                                println!(
                                    "{}",
                                    "Login seccessful. Great to see you again!".on_bright_green()
                                );
                                val
                            }

                            Err(e) => {
                                println!("{}", format!("{e}").bright_red());
                                continue;
                            }
                        };

                        All::User(holder)
                    }

                    2 => {
                        let holder = match ResturantOwner::login() {
                            Ok(val) => {
                                println!(
                                    "{}",
                                    "Login seccessful. Great to see you again!".on_bright_green()
                                );
                                val
                            }

                            Err(e) => {
                                println!("{}", format!("{e}").bright_red());
                                continue;
                            }
                        };

                        All::ResturantOwner(holder)
                    }

                    3 => {
                        let holder = match Admin::login() {
                            Ok(val) => {
                                println!(
                                    "{}",
                                    "Login seccessful. Great to see you again!".on_bright_green()
                                );
                                val
                            }

                            Err(e) => {
                                println!("{}", format!("{e}").bright_red());
                                continue;
                            }
                        };

                        All::Admin(holder)
                    }

                    _ => {
                        continue;
                    }
                };
                break role;
            }

            _ => {}
        }
    };

    match role_of_the_user {
        All::User(mut user) => {
            loop {
                match user.show_command_menu_and_handle() {
                    Ok(str) => {
                        if str == "done" || str == "Done" {
                            break;
                        } else if str == "Empty" {
                        } else {
                            println!("{}", str.bright_green());
                        }
                    }

                    Err(e) => {
                        println!("{}", format!("{e}").bright_red());
                        continue;
                    }
                }
            }
            println!(
                "{}{}",
                "Your taste buds will miss us! Until next time.".on_bright_green(),
                "😁🫶"
            );
        }

        All::ResturantOwner(mut ro) => {
            loop {
                match ro.show_command_menu_and_handle() {
                    Ok(str) => {
                        if str == "done" || str == "Done" {
                            break;
                        } else if str == "Empty" {
                        } else {
                            println!("{}", str.bright_green());
                        }
                    }

                    Err(e) => {
                        println!("{}", format!("{e}").bright_red());
                        continue;
                    }
                }
            }
            println!(
                "{}{}",
                "Thanks for keeping the flavors coming! Wishing you a day full of happy customers"
                    .on_bright_green(),
                " 🍽️"
            );
        }

        All::Admin(mut admin) => {
            loop {
                match admin.show_command_menu_and_handle() {
                    Ok(str) => {
                        if str == "done" || str == "Done" {
                            break;
                        } else if str == "Empty" {
                        } else {
                            println!("{}", str.bright_green());
                        }
                    }

                    Err(e) => {
                        println!("{}", format!("{e}").bright_red());
                        continue;
                    }
                }
            }
            println!(
                "{}{}",
                "All set!Your work keeps everything smoothly - see you next time".on_bright_green(),
                "🫡"
            );
        }
    }
}
