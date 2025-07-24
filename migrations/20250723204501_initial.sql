-- Add migration script here
create table screenshots (
    id integer primary key autoincrement,
    created_at integer not null,
    data blob not null,
    hash text not null,
    saved integer not null default 0
);
