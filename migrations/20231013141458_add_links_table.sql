-- Add migration script here
create table links (
    id uuid primary key,
    user_id uuid references users(id) not null,
    link text not null,
    link_name text,
    inserted_at timestamptz default now() not null,
    updated_at timestamptz default now() not null,
    unique(user_id, link)
)