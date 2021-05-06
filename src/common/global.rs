



pub mod Global
{
	use std::fmt;

	pub enum FileProcessType
	{
		NONE,
		ENCRYPT,
		ENCRYPT_ZIP,
	}

	impl fmt::Display for FileProcessType
	{
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
		{
			match *self
			{
				FileProcessType::NONE => write!(f, "0"),
				FileProcessType::ENCRYPT=> write!(f, "1"),
				FileProcessType::ENCRYPT_ZIP=> write!(f, "2"),
			}
		}
	}
}





