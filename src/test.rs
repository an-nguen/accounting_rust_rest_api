use crate::models::document_type::ModifiedDocumentType;

use super::*;

fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = String::from("postgres://an:FoX13@127.0.0.1/accounting_rs");
    let database_manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(database_manager)
        .expect("Failed to create pool.")
}

#[test]
fn hash_string() {
    let hash = utils::argon2_hash::hash_password(&String::from("test"));
    println!("{}", hash);
}

#[test]
fn test_document_types_repository() {
    let pool = get_pool();
    let conn = pool.get().unwrap();
    let document_type = ModifiedDocumentType {
        name: Option::from(String::from("TEST_TYPE"))
    };
    let result = repository::document_type_repository::create(&conn, document_type)
        .expect("failed to create document type");
    assert_eq!(result.name.unwrap(), String::from("TEST_TYPE"));
    let document_type = ModifiedDocumentType { name: Option::from(String::from("TEST_TYPE2")) };
    let result =
        repository::document_type_repository::
        update(&conn, result.id, document_type).expect("failed to update document type");
    assert_eq!(result.name.unwrap(), String::from("TEST_TYPE2"));
    let result = repository::document_type_repository::delete(&conn, result.id)
        .expect("failed to delete document type");
    assert!(result > 0);
}