

pub mod Process
{
	//use scan::Scan;
	use crate::exe::scan::Scan;

	use std::sync::Arc;
	use std::collections::HashSet;

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
		let mut scan = Scan::new();

		scan.run(
			"./"
		);

		let vv = scan.get_base_files();
		let mut v = vv.upgrade().unwrap();

		let xx = scan.get_origins();
		let x = xx.upgrade().unwrap();

		println!("b f c->{}, {}", v.lock().unwrap().len(), x.lock().unwrap().len());


	}
}


