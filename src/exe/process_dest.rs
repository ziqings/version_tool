

use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::thread;
use std::time;

use super::version::*;
use super::mutex_increase_int::*;

use crate::utils::*;


pub struct ProcessDest
{
	md5_origins: Arc<Mutex<HashMap<String, Arc<RwLock<Vec<Arc<OriginFile>>>>>>>,
}

impl ProcessDest
{
	pub fn new(md5_origins: Arc<Mutex<HashMap<String, Arc<RwLock<Vec<Arc<OriginFile>>>>>>>) -> Self
	{
		return ProcessDest
		{
			md5_origins,
		};
	}

	fn process(ofs: Arc<RwLock<Vec<Arc<OriginFile>>>>)
	{
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
			let torigins = Arc::new(v);

			pool.execute(move || {
				//ProcessDest::process(torigins);
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


