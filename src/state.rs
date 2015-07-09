pub enum State {
	Null,
	Donate { id :String, credit :u32 },
	Restock { crate_size :u32 }
}
