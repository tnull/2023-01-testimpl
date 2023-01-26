fn main() {
	use lightning::impl_writeable_msg;
	use lightning::util::ser::Writeable;
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

	let my_custom_msg = MyCustomMessage { field_1: 1, field_2: true, field_3: "A".to_owned(), tlv_optional_integer: Some(1) };


	let mut my_vec: Vec<u8> = Vec::new();
	my_custom_msg.write(&mut my_vec).unwrap();
	println!("{:?}", my_vec);
}
