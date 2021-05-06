

use std::io::prelude::*;

use std::collections::HashMap;

use std::fs;
use std::path::Path;

use super::version::OriginFile;

use crate::config::Config;
use crate::utils::*;

use std::sync::Mutex;


pub struct Scan
{
	origins: HashMap<String, OriginFile>,
	md5_origins: HashMap<String, Vec<OriginFile>>,
	scanned: HashMap<String, u8>,
}


impl Scan
{
	pub fn new() -> Self
	{
		return Scan
		{
			origins: HashMap::new(),
			md5_origins: HashMap::new(),
			scanned: HashMap::new(),
		};
	}

	fn list_files(scan: &mut Scan, dir: &Path, cb: &dyn Fn(&mut Scan, String))
	{
		if dir.is_dir()
		{
			for entry in fs::read_dir(dir).unwrap()
			{
				let path = entry.unwrap().path();
				Scan::list_files(scan, &path, cb)
			}
		}
		else
		{
			cb(scan, dir.display().to_string());
		}
	}

	pub fn run(&mut self, src_root: &str) -> bool
	{
		let cb = |scan: &mut Scan, p: String| {

			let sp: &str = &p[..];

			if Config::is_ignore_scan(&sp)
			{
				return;
			}

			if Config::is_ignore_scan_file(&sp)
			{
				return;
			}

			if scan.scanned.contains_key(&p)
			{
				println!("scan dup file and ignored->{}", sp);
				return;
			}

			println!("scan dir->{}", p);
			scan.scanned.insert(p, 1);
		};

		let paths = fs::read_dir(src_root).unwrap();
		for path in paths
		{
			let tp = path.unwrap().path();
			if tp.is_dir()
			{
				Scan::list_files(self, &tp, &cb);
			}
			else
			{
				cb(self, tp.display().to_string());
			}
		}

		println!("scaned file count->{}", self.scanned.len());

		if self.scanned.len() > 0
		{
			//let lock = MutexLock::<IncreaseInt>::new();
			let ii = IncreaseInt::new();
			let lock = Mutex::new(ii);
			lock.lock().unwrap().exe();

			println!("lock->{}", lock.lock().unwrap().get());
			lock.lock().unwrap().exe();
			println!("lock->{}", lock.lock().unwrap().get());
		}

		return true;
	}
}

struct IncreaseInt
{
	num: i32,
}

impl MutexDo<i32> for IncreaseInt
{
	fn new() -> Self
	{
		return IncreaseInt
		{
			num: 0
		};
	}

	fn exe(&mut self)
	{
		self.num = self.num + 1;;
	}

	fn get(&self) -> i32
	{
		return self.num;
	}
}

