CREATE DOMAIN CENT AS INT;
CREATE DOMAIN EAN AS VARCHAR(17);

CREATE TABLE balance_card (
	id               SERIAL PRIMARY KEY,
	code             EAN NOT NULL,
	initial_balance  CENT NOT NULL,
	created_on       TIMESTAMP WITHOUT TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE TABLE product (
	id               SERIAL PRIMARY KEY,
	code             EAN NOT NULL,
	quantity         SMALLSERIAL NOT NULL,
	price_per_item   CENT NOT NULL,
	item_name        VARCHAR(16) NOT NULL,
	created_on       TIMESTAMP WITHOUT TIME ZONE DEFAULT (now() AT TIME ZONE 'utc'),
	last_change      TIMESTAMP WITHOUT TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE TABLE donation (
	id               SERIAL PRIMARY KEY,
	balance_card_id  INT REFERENCES balance_card(id) NOT NULL,
	product_id       INT REFERENCES product(id) NOT NULL,
	amount           CENT NOT NULL,
	donated_on       TIMESTAMP WITHOUT TIME ZONE DEFAULT (now() AT TIME ZONE 'utc')
);

CREATE TABLE purchase (
	id               SERIAL PRIMARY KEY,
	product_id       INT REFERENCES product(id),
	quantity         SMALLSERIAL,
	price            CENT
);
