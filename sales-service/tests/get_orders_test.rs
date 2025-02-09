use sales_service; //import the project as a library.
mod common;

#[test]
fn test_add() {
    common::setup();
    let result = sales_service::db::order_repo::add(4, 5);
    assert_eq!(result, 9);
}
