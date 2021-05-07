

use std::io::prelude::*;

use std::collections::HashMap;

use std::fs;
use std::path::Path;

use super::version::OriginFile;

use crate::config::Config;
use crate::utils::*;

use std::sync::Mutex;
use std::sync::Arc;

use std::fs::File;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::io::Write;

//#[macro_use]
//extern crate arrayref;



pub struct Scan
{
	origins: HashMap<String, OriginFile>,
	md5_origins: HashMap<String, Vec<OriginFile>>,
	scanned: HashMap<Arc<String>, u8>,
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

	//fn list_files(scan: &mut Scan, dir: &Path, cb: &dyn Fn(&mut Scan, String))
	//fn list_files(&mut self, dir: &Path, cb: &mut dyn FnMut(String))
	fn list_files(&mut self, dir: &Path)
	{
		if dir.is_dir()
		{
			for entry in fs::read_dir(dir).unwrap()
			{
				let path = entry.unwrap().path();
				//Scan::list_files(scan, &path, cb)
				self.list_files(&path)
			}
		}
		else
		{
			//cb(scan, dir.display().to_string());
			self.cb_list_files(dir.display().to_string());
		}
	}

	fn cb_list_files(&mut self, p:String)
	{
		let sp: &str = &p[..];

		if Config::is_ignore_scan(&sp)
		{
			return;
		}

		if Config::is_ignore_scan_file(&sp)
		{
			return;
		}

		if self.scanned.contains_key(&p)
		{
			println!("scan dup file and ignored->{}", sp);
			return;
		}

		println!("scan dir->{}", p);

		let rp = Arc::new(p);
		self.scanned.insert(rp, 1);
	}

	//fn async_scan_file(&mut self, path: &str)
	fn async_scan_file(path: Arc<String>)
	{
		let relative_path;
		if path.starts_with("./")
		{
			relative_path = path.strip_prefix("./").unwrap();
		}
		else
		{
			relative_path = &path;
		}

		//println!("a s f -> {},   {}", path, relative_path);

		let mut f = File::open(path.to_string()).unwrap();

		let metadata = f.metadata().unwrap();

		let mut buffer = [0u8; 1024];
		let mut rsize = metadata.len() as usize;
		//let mut buffer = vec![0u8; rsize];

		let mut sh = Md5::new();

		while rsize > 0
		{
			let len: usize = f.read(&mut buffer).unwrap();

			/*
			if len > 0
			{
				if len < 1024
				{
					/
					println!("len->{},  {}, {}, {}, {}, {}", len, path.to_string(), buffer[0], buffer[1], buffer[2], buffer[3]);
					let mut arr = vec![0u8; len];
					//arr[..len].clone_from_slice(&buffer);
					//arr.write(&buffer).unwrap();
					//arr.write(&buffer[..]).unwrap();
					//let arr = array_refs!(buffer, 0, len);
					arr.copy_from_slice(&buffer[0..len]);
					println!("arr->{}, {}, {}, {}, {}", path.to_string(), arr[0], arr[1], arr[2], arr[3]);
					sh.input(&arr);
					/
					sh.input(&buffer[0..len]);
				}
				else
				{
					sh.input(&buffer);
				}
			}
			*/

			sh.input(&buffer[0..len]);

			rsize = rsize - len;
		}

		let out_str = sh.result_str();
		println!("md5 str->{},   {}", out_str, path.to_string());
	}

	pub fn run(&mut self, src_root: &str) -> bool
	{
		/*
		//let cb = |scan: &mut Scan, p: String| 
		let mut cb = |p: String| {

		let sp: &str = &p[..];

		if Config::is_ignore_scan(&sp)
		{
		return;
		}

		if Config::is_ignore_scan_file(&sp)
		{
		return;
		}

		if self.scanned.contains_key(&p)
		{
		println!("scan dup file and ignored->{}", sp);
		return;
		}

		println!("scan dir->{}", p);
		self.scanned.insert(p, 1);
		};
		 */

		let pool = ThreadPool::new(4);

		let paths = fs::read_dir(src_root).unwrap();
		for path in paths
		{
			let tp = path.unwrap().path();
			if tp.is_dir()
			{
				//Scan::list_files(self, &tp, &cb);
				self.list_files(&tp);
			}
			else
			{
				//cb(self, tp.display().to_string());
				self.cb_list_files(tp.display().to_string());
			}
		}

		println!("scaned file count->{}", self.scanned.len());

		if self.scanned.len() > 0
		{
			//let lock = MutexLock::<IncreaseInt>::new();
			let ii = IncreaseInt::new();
			let lock = Arc::new(Mutex::new(ii));
			/*
			   lock.lock().unwrap().exe();

			   println!("lock->{}", lock.lock().unwrap().get());
			   lock.lock().unwrap().exe();
			   println!("lock->{}", lock.lock().unwrap().get());
			 */

			for (k, v) in &self.scanned
			{
				let tlock = Arc::clone(&lock);
				let tk = Arc::clone(&k);
				pool.execute(move ||{
						Scan::async_scan_file(tk);
						tlock.lock().unwrap().exe();
						});
			}
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

