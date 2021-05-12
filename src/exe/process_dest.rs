

use std::sync::Mutex;
use std::sync::Arc;
use std::sync::RwLock;

use std::collections::HashMap;
use std::thread;
use std::time;

use std::path::Path;
use std::fs;
use std::fs::File;

use std::io::Read;
use std::io::Write;
use std::io::Seek;

use super::version::*;
use super::mutex_increase_int::*;

use crate::utils::*;
use crate::common::Global;
use crate::config::Config;


pub struct ProcessDest
{
	md5_origins: Arc<Mutex<HashMap<String, Arc<RwLock<Vec<Arc<OriginFile>>>>>>>,
					 dest_root: Arc<String>,
}

impl ProcessDest
{
	pub fn new(md5_origins: Arc<Mutex<HashMap<String, Arc<RwLock<Vec<Arc<OriginFile>>>>>>>) -> Self
	{
		return ProcessDest
		{
			md5_origins,
				dest_root: Arc::new("root".to_string()),
		};
	}

	fn process(dest_root: Arc<String>, ofs: Arc<RwLock<Vec<Arc<OriginFile>>>>)
	{
		let vec = ofs.read().unwrap();
		println!("vec len->{}", vec.len());

		/*
		let rof = Arc::try_unwrap(Arc::clone(&vec[0]));

		match rof
		{
			Ok(of) => {
				println!("of->{}", of.path);
			},
				Err(err) => {println!("err");},
		};
		*/

		//let of = Arc::try_unwrap(Arc::clone(&vec[0])).unwrap();
		//println!("of->{}", of.path);

		let of = &vec[0];
		println!("of->{}, {}", of.path, of.full_path);

		let mut index = of.path.rfind('/');
		if let Some(ii) = index
		{
			if (ii < 0)
			{
				index = of.path.rfind('\\');
			}
		}

		let mut relative_path: &str = "";
		if let Some(ii) = index
		{
			if (ii >= 0)
			{
				let sp = of.path.split_at(ii + 1);
				relative_path = sp.0;
			}
		}

		let dir_path = format!("{}/{}", dest_root, relative_path);
		println!("r p->{}", dir_path);

		let p = Path::new(&dir_path);
		if !p.exists()
		{
			fs::create_dir_all(p).unwrap();
		}

		let tp = Path::new(&of.path);

		let rext = tp.extension();
		let ext: String;
		match rext
		{
			Some(t) => {
				ext = String::from(t.to_str().unwrap());
			},
				None => {ext = "".to_string()},
		}
		let file_name = tp.file_name().unwrap();

		println!("ext->{:?}, {:?}", file_name, ext);

		let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

		let fpt = Config::check_file_fpt(&of.full_path);
		if fpt == Global::FileProcessType::ENCRYPT_ZIP
		{
			//let mut buf: Vec<u8> = Vec::with_capacity(data.len());
			//let mut buf = [0u8; 65535];
			let mut buf = [0u8; 2048];
			let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf[..]));
			let p = format!("./tmp/{}", of.md5);
			zip.start_file(p, options);
			//let data = fs::read(&of.full_path).unwrap();
			//zip.write(&data[..]);

			let mut f = File::open(&of.full_path).unwrap();

			let metadata = f.metadata().unwrap();

			let mut rsize = metadata.len() as usize;
			let fsize = rsize as u32;
			let mut rbuf = [0u8; 1024];

			while rsize > 0
			{
				let len = f.read(&mut rbuf).unwrap();
				zip.write(&rbuf[0..len]);
				rsize = rsize - len;
			}

			zip.finish();
		}
	}

	pub fn run(&self) -> bool
	{
		let lii = IncreaseInt::new();
		let lock = Arc::new(Mutex::new(lii));

		let pool = ThreadPool::new(4);

		for (k, v) in self.md5_origins.lock().unwrap().iter()
		{
			println!("process dest->{}", k);

			let tlock = Arc::clone(&lock);
			//let tk = Arc::new(k);
			let torigins = Arc::clone(v);
			let tdest_root = Arc::clone(&self.dest_root);

			pool.execute(move || {
					ProcessDest::process(tdest_root, torigins);
					tlock.lock().unwrap().exe();
					});
		}

		let total = self.md5_origins.lock().unwrap().len() as i32;
		println!("process dest before wait->{}, {}", lock.lock().unwrap().get(), total);
		while(lock.lock().unwrap().get() != total)
		{
			let ten = time::Duration::from_millis(10);
			thread::sleep(ten);
		}
		println!("process dest wait over->{}, {}", lock.lock().unwrap().get(), total);

		return true;
	}
}


