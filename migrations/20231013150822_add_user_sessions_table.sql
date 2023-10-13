-- Add migration script here
create table user_sessions(
    id uuid primary key,
    user_id uuid references users(id) not null,
    token text not null,
    expires_at timestamptz not null,
    inserted_at timestamptz default now() not null,
    updated_at timestamptz default now() not null
)
