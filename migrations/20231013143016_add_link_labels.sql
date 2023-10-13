-- Add migration script here
create table link_labels(
    link_label varchar(255) not null,
    link_id uuid references links(id) not null,
    primary key(link_label, link_id)
)
