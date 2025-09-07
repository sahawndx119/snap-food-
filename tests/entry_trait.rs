use mini_projects3::{handlres_entry::*, model::*};

#[test]
fn login_test_user_correct() {
    assert!(User::login().is_ok());
} //works successfully✅

#[test]
fn login_test_user_wrong_password() {
    assert!(User::login().is_err());
} //works successfully✅

#[test]
fn login_another_role_for_user() {
    assert!(User::login().is_err());
    //we give it a correct info but it's role is not user..so we can ensure that out function
    //is working successfully
} //works successfully✅

#[test]
fn login_test_admin_correct() {
    assert!(Admin::login().is_ok());
} //works successfully✅

#[test]
fn login_test_admin_wrong_password() {
    assert!(Admin::login().is_err());
} //works successfully✅

#[test]
fn login_another_role_for_admin() {
    assert!(Admin::login().is_err());
}

#[test]
fn login_test_resturantowner_correct() {
    assert!(ResturantOwner::login().is_ok());
} //works successfully✅

#[test]
fn login_test_resturantowner_wrong_password() {
    assert!(ResturantOwner::login().is_err());
} //works successfully✅

#[test]
fn login_another_role_for_resturantowner() {
    assert!(ResturantOwner::login().is_err());
}
