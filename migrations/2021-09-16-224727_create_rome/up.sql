-- Your SQL goes here
CREATE TABLE romes (
    uuid BINARY(16) PRIMARY KEY,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NULL,
    code VARCHAR(100) NOT NULL,
    label VARCHAR(100) NOT NULL,
    description VARCHAR(1000)
);

