-- Your SQL goes here
CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    author VARCHAR NOT NULL ,
    contest VARCHAR NOT NULL ,
    statement VARCHAR NOT NULL ,
    rating INT NOT NULL
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL ,
    last_name VARCHAR NOT NULL ,
    school VARCHAR NOT NULL ,
    bio VARCHAR NOT NULL ,
    teacher VARCHAR NOT NULL
);

CREATE TABLE submissions (
    id SERIAL PRIMARY KEY ,
    user_id INTEGER REFERENCES users(id) not null ,
    problem_id INTEGER REFERENCES problems(id) not null ,
    source_code VARCHAR NOT NULL,
    score INTEGER NOT NULL,
    language VARCHAR NOT NULL
)