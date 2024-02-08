-- Your SQL goes here
create table wallet (
    id INT GENERATED ALWAYS AS IDENTITY,
    its INT,
    primary key(id)
);

create table person (
    id INT GENERATED ALWAYS AS IDENTITY,
    wallet INT not null,
    created timestamp (3),
    primary key(id),
    constraint fk_wallet
        foreign key(wallet)
            references wallet(id)
);