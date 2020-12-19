table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        itn -> Nullable<Varchar>,
        okonkh -> Nullable<Varchar>,
        okpo -> Nullable<Varchar>,
        bic -> Nullable<Varchar>,
        bank_account -> Nullable<Varchar>,
    }
}

table! {
    currencies (id) {
        id -> Int4,
        name -> Varchar,
        short_name -> Nullable<Varchar>,
    }
}

table! {
    currencies_measurements (id) {
        id -> Int4,
        currency_id -> Nullable<Int4>,
        value -> Numeric,
        date -> Nullable<Date>,
    }
}

table! {
    document_items (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        document_id -> Nullable<Int4>,
        series_code -> Nullable<Numeric>,
        quantity -> Numeric,
        price -> Numeric,
        total -> Numeric,
        currency_id -> Nullable<Int4>,
        unit_id -> Nullable<Int4>,
    }
}

table! {
    document_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    documents (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        edited_at -> Nullable<Timestamp>,
        is_valid -> Nullable<Bool>,
        sender_company_id -> Nullable<Int4>,
        middleman_company_id -> Nullable<Int4>,
        recipient_company_id -> Nullable<Int4>,
        document_type_id -> Nullable<Int4>,
        document_number -> Nullable<Int4>,
        comment -> Nullable<Varchar>,
    }
}

table! {
    privileges (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        short_name -> Nullable<Varchar>,
        full_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        vendor_code -> Nullable<Numeric>,
        purchase_price -> Nullable<Numeric>,
        selling_price -> Nullable<Numeric>,
        default_currency -> Nullable<Int4>,
        default_unit -> Nullable<Int4>,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    roles_privileges (role_id, privilege_id) {
        role_id -> Int4,
        privilege_id -> Int4,
        allow -> Nullable<Bool>,
    }
}

table! {
    units (id) {
        id -> Int4,
        name -> Varchar,
        short_name -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        birthday -> Nullable<Date>,
        valid_to -> Nullable<Date>,
    }
}

table! {
    users_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
    }
}

joinable!(currencies_measurements -> currencies (currency_id));
joinable!(document_items -> currencies (currency_id));
joinable!(document_items -> documents (document_id));
joinable!(document_items -> products (product_id));
joinable!(document_items -> units (unit_id));
joinable!(documents -> document_types (document_type_id));
joinable!(products -> currencies (default_currency));
joinable!(products -> units (default_unit));
joinable!(roles_privileges -> privileges (privilege_id));
joinable!(roles_privileges -> roles (role_id));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    companies,
    currencies,
    currencies_measurements,
    document_items,
    document_types,
    documents,
    privileges,
    products,
    roles,
    roles_privileges,
    units,
    users,
    users_roles,
);
