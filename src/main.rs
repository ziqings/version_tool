


//include!("thread_pool.rs");

//include!("zltext.rs");

use std::io::prelude::*;
//use std::fs;

//mod ThreadPool;

mod utils;
mod config;

use crate::utils::ThreadPool;
use crate::config::Config;



fn main() 
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
    });
}
