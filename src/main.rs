


//include!("thread_pool.rs");

//include!("zltext.rs");

use std::io::prelude::*;
//use std::fs;

//mod ThreadPool;

mod common;
mod utils;
mod config;
mod exe;
mod test;


use crate::test::Test;

fn main() 
{
	Test::run();
}
