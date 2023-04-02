-- Your SQL goes here
create table participates (
    uid INTEGER REFERENCES users(id),
    cid INTEGER REFERENCES contest(id),
    score INTEGER not null,
    official BOOLEAN not null,
    PRIMARY KEY (uid, cid)
)