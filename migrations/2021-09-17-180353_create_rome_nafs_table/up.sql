-- Your SQL goes here
CREATE TABLE rome_nafs (
    uuid BINARY(16) PRIMARY KEY,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NULL,
    rome_uuid BINARY(16) not null,
	naf_uuid BINARY(16) not null,

    foreign key (rome_uuid) references romes(uuid),
	foreign key (naf_uuid) references nafs(uuid)
);

ALTER TABLE `rome_nafs` ADD UNIQUE `rome_nafs_unique_rome_id_naf_id` (`rome_uuid`, `naf_uuid`);
