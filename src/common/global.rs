



pub mod Global
{
	use std::fmt;
	use std::cmp;

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

	impl cmp::PartialEq for FileProcessType
	{
		fn eq(&self, r: &FileProcessType) -> bool
		{
			return self == r;
		}
	}
}





