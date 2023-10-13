-- Add migration script here
create table users (
    id uuid primary key,
    username varchar(255) not null unique,
    password_hash text not null,
    inserted_at timestamptz default now() not null,
    updated_at timestamptz default now() not null
);
