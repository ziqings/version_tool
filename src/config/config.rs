

pub mod Config
{
	use std::fs;
	use crate::utils::ZLText;

	static mut m_ignore_scan_extension: Vec<String> = Vec::new();


	pub fn init()
	{
		let cstr = fs::read_to_string("config.zl").unwrap();

		let zl = ZLText::new(&cstr[..]);

		let lst = zl.read("ignore_scan_extension");

		//m_ignore_scan_extension = Vec::from_capacity(lst.len());

		for ii in lst
		{
			unsafe
			{
				m_ignore_scan_extension.push(String::from(ii));
			}
		}
	}

	pub fn is_ignore_scan_file(path:&str) -> bool
	{
		let arr: Vec<&str> = path.split('.').collect();
		if arr.len() > 1
		{
			let ext = format!(".{}", arr[1]);

			unsafe
			{
				for ii in &m_ignore_scan_extension
				{
					if *ii == ext
					{
						return true;
					}
				}
			}
		}

		return false;
	}
}



