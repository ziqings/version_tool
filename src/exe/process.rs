

pub mod Process
{
	use crate::exe::scan::Scan;
	use crate::exe::process_dest::ProcessDest;

	use std::sync::Arc;
	use std::collections::HashSet;
	use std::path::Path;
	use std::fs;

	use rustc_serialize::json;

	use crate::exe::version::*;

	fn read_full_version() -> Option<FullVersion>
	{
		let v = Path::new("full_version.json");
		if v.is_file()
		{
			let fstr = fs::read_to_string("").unwrap();
			let fv: FullVersion = json::decode(&fstr).unwrap();

			return Some(fv);
		}
		else
		{
			return Option::<FullVersion>::None;
		}
	}

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
		let fv = read_full_version();

		let mut scan = Scan::new(fv);

		scan.run(
			"./"
		);

		let bases = scan.get_base_files().upgrade().unwrap();
		let origins = scan.get_origins().upgrade().unwrap();
		let md5_origins = scan.get_md5_origins().upgrade().unwrap();

		println!("b f c->{}, {}, {}", bases.lock().unwrap().len(), origins.lock().unwrap().len(), md5_origins.lock().unwrap().len());

		if origins.lock().unwrap().len() == 0
		{
			println!("no resource need to be updated");
		}
		else
		{
			println!("need updating count->{}", origins.lock().unwrap().len());

			for (k, v) in origins.lock().unwrap().iter()
			{
				println!("need updating->{}", k);
			}
		}

		let torigins = Arc::clone(&origins);
		let mut pdest = ProcessDest::new(torigins);

		pdest.run();
	}
}


