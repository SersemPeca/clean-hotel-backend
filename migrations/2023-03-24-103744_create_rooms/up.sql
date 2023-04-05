-- Your SQL goes here
CREATE TABLE rooms (
	id INTEGER NOT NULL,
	cleaner INTEGER,
    clean BOOLEAN NOT NULL DEFAULT FALSE,
    description VARCHAR,
	CONSTRAINT rooms_pk
		PRIMARY KEY (id)
	CONSTRAINT cleaner_fk
		FOREIGN KEY (cleaner)
			REFERENCES cleaners(id)
			ON DELETE SET NULL
)
