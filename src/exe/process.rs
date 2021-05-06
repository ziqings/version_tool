

pub mod Process
{
	//use scan::Scan;
	use crate::exe::scan::Scan;

	pub fn process(
			src_path: &str, 
			dest_path: &str, 
			base_path: &str, 
			cache_file: &str, 
			config_file: &str, 
			major: &str, 
			lang: &str
			)
	{
		let scan = Scan::new();
		scan.run(
			"./"
		);
	}
}


