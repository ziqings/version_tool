

use crate::utils::*;

pub struct IncreaseInt
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

