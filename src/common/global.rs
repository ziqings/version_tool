



pub mod Global
{
	use std::fmt;
	use std::cmp;

#[derive(PartialEq)]
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

	/*
	impl cmp::PartialEq for FileProcessType
	{
		fn eq(&self, r: &FileProcessType) -> bool
		{
			println!("check eq");
			return *self as i32  == *r as i32;
		}
	}
	*/
}





