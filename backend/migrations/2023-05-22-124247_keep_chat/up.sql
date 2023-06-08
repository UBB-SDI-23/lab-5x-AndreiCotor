create table chat (
    id serial primary key,
    nickname varchar not null,
    message varchar not null,
    uid int references users(id)
);