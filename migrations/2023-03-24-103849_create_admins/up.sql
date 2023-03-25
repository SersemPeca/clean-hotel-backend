-- Your SQL goes here
CREATE TABLE admins (
	id	 INTEGER NOT NULL,
	username VARCHAR NOT NULL,
	password VARCHAR NOT NULL,
	name 	 VARCHAR NOT NULL,
	CONSTRAINT admins_pk
		PRIMARY KEY (id)
)
