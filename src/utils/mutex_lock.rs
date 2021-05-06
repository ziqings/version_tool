

//use std::sync::Mutex;



pub trait MutexDo<T1>
{
	fn new() -> Self;
	fn exe(&mut self);
	fn get(&self) -> T1;
}


/*
pub struct MutexLock<T>
{
	lock: Mutex<T>,
}

impl<T> MutexLock<T> where T: MutexDo<T1>
{
	pub fn new() -> Self
	{
		let t = T::new();

		return MutexLock
		{
			lock: Mutex::new(t),
		};
	}

	pub fn exe(&self)
	{
		self.lock.lock().unwrap().exe();
	}

	pub fn get(&self) -> T1
	{
		return self.lock.lock().unwrap().get();
	}
}
*/



