CREATE TABLE UserCredentials (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    password varchar NOT NULL
);

create index idx_username on UserCredentials(username);