create table pagoptions (
    id serial primary key,
    pages int not null default 10
);

insert into pagoptions (id, pages) values (1, 10);