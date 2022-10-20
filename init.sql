drop table if exists account;

create table account(
    a_id serial primary key,
    a_username char(50) not null,
    a_email char(50) not null,
    a_password char(50) not null
);