-- Your SQL goes here
create table documents (
    id serial primary key ,
    created_at timestamp default current_timestamp,
    edited_at  timestamp default current_timestamp,
    is_valid   boolean   default true,
    sender_company_id  integer references companies(id) on update cascade on delete set null,
    middleman_company_id  integer references companies(id) on update cascade on delete set null,
    recipient_company_id  integer references companies(id) on update cascade on delete set null ,
    document_type_id      integer references document_types(id) on update cascade on delete set null,
    document_number       integer ,
    comment               varchar(512)
)