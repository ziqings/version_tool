

use std::io::prelude::*;

use std::collections::HashMap;

use std::fs;
use std::path::Path;

use super::version::OriginFile;

use crate::config::Config;


pub struct Scan
{
	origins: HashMap<String, OriginFile>,
	md5_origins: HashMap<String, Vec<OriginFile>>,
}


impl Scan
{
	pub fn new() -> Self
	{
		return Scan
		{
			origins: HashMap::new(),
			md5_origins: HashMap::new(),
		};
	}

	fn list_files(dir: &Path, cb: &dyn Fn(String))
	{
		if dir.is_dir()
		{
			for entry in fs::read_dir(dir).unwrap()
			{
				let path = entry.unwrap().path();
				Scan::list_files(&path, cb)
			}
		}
		else
		{
			cb(dir.display().to_string());
		}
	}

	pub fn run(&self, src_root: &str) -> bool
	{
		let cb = |p: String| {
			if Config::is_ignore_scan(&p[..])
			{
				return;
			}

			if Config::is_ignore_scan_file(&p[..])
			{
				return;
			}

			println!("scan dir->{}", p);
		};

		let paths = fs::read_dir(src_root).unwrap();
		for path in paths
		{
			let tp = path.unwrap().path();
			if tp.is_dir()
			{
				Scan::list_files(&tp, &cb);
			}
			else
			{
				cb(tp.display().to_string());
			}
		}


		return true;
	}
}

