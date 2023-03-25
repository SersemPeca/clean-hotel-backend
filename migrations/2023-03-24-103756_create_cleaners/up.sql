-- Your SQL goes here
CREATE TABLE cleaners (
	id	 INTEGER NOT NULL,
	username VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	name 	 VARCHAR NOT NULL,
	CONSTRAINT cleaners_pk
		PRIMARY KEY (id),
	CONSTRAINT cleaners_username_unique
		UNIQUE (username)
)
