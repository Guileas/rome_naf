-- Your SQL goes here
CREATE TABLE specialtys (
    uuid BINARY(16) PRIMARY KEY,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NULL,
    label VARCHAR(255) NOT NULL,
    description VARCHAR(1000),
    naf_id BINARY(16) NOT NULL,

    foreign key (naf_id) references nafs(uuid)
);
