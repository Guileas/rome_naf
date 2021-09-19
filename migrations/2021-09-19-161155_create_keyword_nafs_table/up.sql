-- Your SQL goes here
CREATE TABLE keyword_nafs (
    uuid BINARY(16) PRIMARY KEY,
    created_at TIMESTAMP(6) NOT NULL,
    updated_at TIMESTAMP(6) NULL,
    keyword_uuid BINARY(16) not null,
	naf_uuid BINARY(16) not null,

    foreign key (keyword_uuid) references keywords(uuid),
	foreign key (naf_uuid) references nafs(uuid)
);

ALTER TABLE `keyword_nafs` ADD UNIQUE `rome_nafs_unique_keyword_id_naf_id` (`keyword_uuid`, `naf_uuid`);
