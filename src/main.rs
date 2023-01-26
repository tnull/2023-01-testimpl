fn main() {
	use lightning::impl_writeable_msg;
	struct MyCustomMessage {
		pub field_1: u32,
		pub field_2: bool,
		pub field_3: String,
		pub tlv_optional_integer: Option<u32>,
	}

	impl_writeable_msg!(MyCustomMessage, {
		field_1,
		field_2,
		field_3
	}, {
		(1, tlv_optional_integer, option),
	});
}
