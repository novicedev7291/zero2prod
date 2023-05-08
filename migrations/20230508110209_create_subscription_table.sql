create table subscriptions(
    id uuid not null,
    primary key(id),
    name text not null,
    email text not null unique,
    subscribed_at timestamptz not null-- Add migration script here
);
