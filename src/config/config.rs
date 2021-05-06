

pub mod Config
{
	use std::fs;
	use crate::utils::ZLText;

	static mut m_ignore_scan_extension : Vec<String> = Vec::new();
	static mut m_ignore_zip_extension : Vec<String> = Vec::new();
	static mut m_ignore_encrypt_extension : Vec<String> = Vec::new();
	static mut m_base_regex : Vec<String> = Vec::new();
	static mut m_ignore_base_regex : Vec<String> = Vec::new();


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

		let ize = zl.read("ignore_zip_extension");
		for ii in ize
		{
			unsafe
			{
				m_ignore_zip_extension.push(String::from(ii));
			}
		}

		let iee = zl.read("ignore_encrypt_extension");
		for ii in iee
		{
			unsafe
			{
				m_ignore_encrypt_extension.push(String::from(ii));
			}
		}

		let br = zl.read("base_regex");
		for ii in br
		{
			unsafe
			{
				//m_base_regex.push(String::from(ii));
				m_base_regex.push(ii.to_lowercase());
			}
		}

		let ibr = zl.read("ignore_base_regex");
		for ii in ibr
		{
			unsafe
			{
				//m_ignore_base_regex.push(String::from(ii));
				m_ignore_base_regex.push(ii.to_lowercase());
			}
		}
	}

	pub fn is_ignore_scan_file(path : &str) -> bool
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

	pub fn is_in_base(path : &str) -> bool
	{
		let lp = path.to_lowercase();
		//let lp_arr = lp.into_bytes();
		let slp: &str = &lp[..];

		//let regex = pcre2::bytes::Regex::new(&m_base_regex).unwrap();

		unsafe
		{
			for ii in &m_base_regex
			{
				println!("base regex->{}", ii);
				//let re = pcre2::bytes::Regex::new(&ii[..]).unwrap().is_match(&lp.into_bytes());
				//let re = pcre2::bytes::Regex::new(&ii[..]).unwrap().is_match(&lp_arr);
				let reg = regex::Regex::new(ii).unwrap();
				let re = reg.is_match(&slp);
				/*
				match re
				{
					Ok(r) => 
					{
						return true; 
					},
						_ => {},
				}
				*/
				if re 
				{
					return re;
				}
			}
		}

		return false;
	}
}



