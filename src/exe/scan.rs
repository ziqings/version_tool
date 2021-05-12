

use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

use std::fs;
use std::path::Path;

use super::version::*;
use super::mutex_increase_int::*;

use crate::config::Config;
use crate::utils::*;

use std::sync::Mutex;
use std::sync::Arc;
use std::sync::Weak;
use std::sync::RwLock;

use std::rc::Rc;

use std::fs::File;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::io::Write;

use std::thread;
use std::time;

//#[macro_use]
//extern crate arrayref;



pub struct Scan
{
	origins: Arc<Mutex<HashMap<String, Arc<OriginFile>>>>,
	md5_origins: Arc<Mutex<HashMap<String, Arc<RwLock<Vec<Arc<OriginFile>>>>>>>,
	scanned: HashMap<Arc<String>, u8>,
	base_files: Arc<Mutex<HashSet<String>>>,
	full_version: Option<FullVersion>,
	version_files: HashMap<Rc<String>, Rc<FullVersionFile>>,
}


impl Scan
{
	pub fn new(fv: Option<FullVersion>) -> Self
	{
		return Scan
		{
			origins: Arc::new(Mutex::new(HashMap::new())),
			md5_origins: Arc::new(Mutex::new(HashMap::new())),
			scanned: HashMap::new(),
			base_files: Arc::new(Mutex::new(HashSet::new())),
			full_version: fv,
			version_files: HashMap::new(),
		};
	}

	pub fn get_md5_origins(&self) -> Weak<Mutex<HashMap<String, Arc<RwLock<Vec<Arc<OriginFile>>>>>>>
	{
		return Arc::downgrade(&self.md5_origins);
	}

	pub fn get_base_files(&self)-> Weak<Mutex<HashSet<String>>>
	{
		return Arc::downgrade(&self.base_files);
	}

	pub fn get_origins(&self) -> Weak<Mutex<HashMap<String, Arc<OriginFile>>>>
	{
		return Arc::downgrade(&self.origins);
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

	fn async_scan_file(
			path: Arc<String>, 
			torigins: Arc<Mutex<HashMap<String, Arc<OriginFile>>>>, 
			tmd5_origins: Arc<Mutex<HashMap<String, Arc<RwLock<Vec<Arc<OriginFile>>>>>>>,
			tbase_files: Arc<Mutex<HashSet<String>>>
			)
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
		let fsize = rsize as u32;
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
		let md5_str = String::from(&out_str[..]);
		//println!("md5 str->{},   {}", out_str, path.to_string());

		let in_base = Config::is_in_base(&format!("/{}", relative_path)[..]);

		let of = OriginFile::new(
				path.to_string(),
				relative_path.to_string(),
				fsize,
				out_str,
				);
		let aof = Arc::new(of);
		let aaof = Arc::clone(&aof);

		let mut tor = torigins.lock().unwrap();

		if in_base
		{
			let mut tbases = tbase_files.lock().unwrap();
			tbases.insert(relative_path.to_string());
		}

		tor.insert(relative_path.to_string(), aof);

		let mut md5_tor = tmd5_origins.lock().unwrap();

		let md5_key = Scan::get_md5_key(&relative_path, &md5_str[..]);
		println!("md5 key->{}", md5_key);

		if !md5_tor.contains_key(&md5_key)
		{
			md5_tor.insert(md5_key.to_string(), Arc::new(RwLock::new(Vec::new())));
		}

		let mut lst = md5_tor.get_mut(&md5_key).unwrap().write().unwrap();

		lst.push(aaof);
	}

	fn get_md5_key(relative_path: &str, md5: &str) -> String
	{
		//return "".to_string();
		println!("get md5 key 0->{}", relative_path);
		let index = relative_path.rfind('/');;

		if let Some(ii) = index
		{
			let sp = relative_path.split_at(ii + 1);

			return format!("{}{}", sp.0, md5);
		}
		else
		{
			return String::from(md5);
		}
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
				let torigins = Arc::clone(&self.origins);
				let tmd5_origins = Arc::clone(&self.md5_origins);
				let tbase_files = Arc::clone(&self.base_files);
				pool.execute(move ||{
						Scan::async_scan_file(tk, torigins, tmd5_origins, tbase_files);
						tlock.lock().unwrap().exe();
						});
			}

			let total = self.scanned.len() as i32;

			println!("before wait->{}, {}", lock.lock().unwrap().get(), total);

			while(lock.lock().unwrap().get() != total)
			{
				let ten = time::Duration::from_millis(10);
				thread::sleep(ten);
			}

			println!("scan wait over->{}, {}", lock.lock().unwrap().get(), self.base_files.lock().unwrap().len());

			self.scan_over();
		}

		return true;
	}

	fn scan_over(&mut self)
	{
		if let Some(fv) = &self.full_version
		{
			for item in &fv.version_files
			{
				let p = Rc::clone(&item.version_file.path);
				let kp = Rc::clone(&p);
				let ph = Rc::try_unwrap(p).unwrap();
				if let Some(v) = self.origins.lock().unwrap().get(&ph)
				{
					if v.md5 == item.origin_md5
					{
						let c_item = Rc::clone(item);
						self.version_files.insert(kp, c_item);

						self.origins.lock().unwrap().remove(&ph);

						let md5_key = Scan::get_md5_key(&item.version_file.path, &v.md5);
						self.md5_origins.lock().unwrap().remove(&md5_key);
					}
				}
			}
		}
	}
}


