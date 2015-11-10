use postgres::Connection;

use error::DbError;

pub type Cent = i32;

pub const QUERY_CARD_BALANCE : &'static str = "
	SELECT balance_card.initial_balance - SUM(donation.amount)
	FROM balance_card INNER JOIN donation
		ON balance_card.id = donation.balance_card_id
	WHERE balance_card.code = $1
	GROUP BY balance_card.id
	";

pub fn get_card_balance(ean :&str, conn :&Connection) -> Result<Cent, DbError> {
	let stmt = conn.prepare_cached(QUERY_CARD_BALANCE).unwrap();
	for row in stmt.query(&[&ean]).unwrap() {
		return Ok(row.get(0));
	}

	Err(DbError::NoData(ean.to_owned()))
}
