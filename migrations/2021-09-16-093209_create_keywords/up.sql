-- Your SQL goes here
CREATE TABLE keywords (
    uuid BINARY(16) PRIMARY KEY,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NULL,
    label VARCHAR(100) NOT NULL
);

