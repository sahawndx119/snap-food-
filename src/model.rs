//in the name of God//

pub use chrono::Local;
pub use serde::{self, Deserialize, Serialize};
pub use std::fmt::Display;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub enum Role {
    User,
    Admin,
    ResturantOwner,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role,
    pub order: Vec<UserOrder>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Admin {
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResturantOwner {
    pub owner: String,
    pub password: String,
    pub role: Role,
    pub resturant: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub username: String,
    pub resturant: String,
    pub items: Vec<Item>,
    pub total_price: f64,
    pub datetime: String,
    pub status : OrderStatus,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserOrder {
    pub resturant: String,
    pub items: Vec<Item>,
    pub total_price: f64,
    pub datetime: String,
    pub status : OrderStatus,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Resturant {
    pub name: String,
    pub owner: String,
    pub category: Category,
    pub menu: Vec<Item>,
    pub discount : u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct Item {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorType {
    WrongPassword(String),
    ReadFromFile(String),
    JsonEncode(String),
    JsonDecode(String),
    NotFound(String),
    InvalidCommand(String),
    WriteOnFile(String),
    EmptyFile(String),
    ReapetedName(String),
    WrongResturant(String),
    HigherQuantity(String),
    CanceledOrder(String),
    Password(String),
    NoDiscount(String),
    BackOption,
    NGH, //never gonna happend//
}

#[derive(Serialize, Deserialize)]
pub enum All {
    User(User),
    ResturantOwner(ResturantOwner),
    Admin(Admin),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Category {
    FastFood,
    TraditionalFood,
    Dessert,
    Persian,
    SeaFood,
    Vegan,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::WrongPassword(val) => write!(f, "{}", val),
            ErrorType::ReadFromFile(val) => write!(f, "{}", val),
            ErrorType::JsonEncode(val) => write!(f, "{}", val),
            ErrorType::JsonDecode(val) => write!(f, "{}", val),
            ErrorType::NotFound(val) => write!(f, "{}", val),
            ErrorType::InvalidCommand(val) => write!(f, "{}", val),
            ErrorType::WriteOnFile(val) => write!(f, "{}", val),
            ErrorType::EmptyFile(val) => write!(f, "{}", val),
            ErrorType::ReapetedName(val) => write!(f, "{}", val),
            ErrorType::WrongResturant(val) => write!(f, "{}", val),
            ErrorType::HigherQuantity(val) => write!(f, "{}", val),
            ErrorType::CanceledOrder(val) => write!(f, "{}", val),
            ErrorType::NGH => write!(f, "NOT GONNA HAPPEN"),
            ErrorType::BackOption => write!(f, ""),
            ErrorType::Password(val) => write!(f, "{}" , val),
            ErrorType::NoDiscount(val) => write!(f , "{}" , val),
        }
    }
}


impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::FastFood => write!(f , "Fast Food"),
            Category::TraditionalFood => write!(f , "Traditional Food"),
            Category::Dessert => write!(f , "Dessert"),
            Category::Persian => write!(f , "Persin"),
            Category::SeaFood => write!(f , "Sea Food"),
            Category::Vegan => write!(f , "Vegan"),
        }
    }
}


pub enum Security {
    Weak,
    Good,
    Powerful,
    AllSecure,
}

impl Display for Security{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Security::Weak => write!(f , "Security [🔴⚪⚪⚪] Weak"),
            Security::Good => write!(f , "Security [🟠🟠⚪⚪] Good"),
            Security::Powerful => write!(f , "Security [🟡🟡🟡⚪] Powerful"),
            Security::AllSecure => write!(f , "Security [🟢🟢🟢🟢] All Secure ✅"),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Canceled,
    Preparing,
    Delivered,
}


impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Canceled => write!(f , "Canceled"),
            OrderStatus::Preparing => write!(f , "Preparing"),
            OrderStatus::Delivered => write!(f , "Delivered"),
        }
    }
}