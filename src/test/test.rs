


pub mod Test
{
	use std::io::prelude::*;

	use crate::utils::ThreadPool;
	use crate::config::Config;

	use rustc_serialize::json;

	use crate::exe::*;


	pub fn run()
	{
		let pool = ThreadPool::new(4);
		pool.execute(||{
				println!("thread execute");

				/*
				   let cstr = fs::read_to_string("config.zl").unwrap();

				   let zl = ZLText::new(&cstr[..]);
				   let lst = zl.read("test1");
				   for ii in lst
				   {
				   println!("v->{}", ii);
				   }
				 */

				Config::init();

				let re = Config::is_ignore_scan_file("xxx.svn");
				println!("re->{}", re);

				let re1 = Config::is_ignore_scan_file("dkdkdkdkkdkdkdk");
				println!("re1->{}", re1);

				let re2 = Config::is_in_base("audio/test");
				println!("re2->{}", re2);

				let re3 = Config::is_in_base("/lua/test");
				println!("re3->{}", re3);

				let re4 = Config::check_file_fpt("xxx/fff.bundle");
				println!("re4->{}", re4);

				let vf = VersionFile::new
					(
					 "aa".to_string(),
					 "bb".to_string(),
					 1,
					 2,
					 1,
					 vec!["1".to_string(), "2".to_string()],
					);

				let encode_str = json::encode(&vf).unwrap();
				println!("json encode->{}", encode_str);

				let vvf: VersionFile = json::decode(&encode_str).unwrap();
				println!("vvf md5->{}", vvf.get_md5());
		});

	}
}



