

use std::rc::Rc;


#[derive(RustcDecodable, RustcEncodable)]
pub struct VersionFile
{
	pub path: Rc<String>,
		pub md5: String,
		pub zip_size: i32,
		pub origin_size: i32,
		pub fpt: i32,
		pub split_files: Vec<String>,
}

impl VersionFile
{
	pub fn new(
			path: String,
			md5: String,
			zip_size: i32,
			origin_size: i32,
			fpt: i32,
			split_files: Vec<String>,
			) -> Self
	{
		return VersionFile
		{
			path: Rc::new(path),
			md5,
			zip_size,
			origin_size,
			fpt,
			split_files,
		};
	}

	pub fn get_md5(&self) -> &str
	{
		return &self.md5[..];
	}
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct FullVersionFile
{
	pub version_file: VersionFile,
		pub origin_md5: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct VersionInfo
{
	pub base_files: Vec<VersionFile>,
		pub extra_files: Vec<VersionFile>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct FullVersion
{
	pub version_files: Vec<Rc<FullVersionFile>>,
}


#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct OriginFile
{
	pub full_path: String,
		pub path: String,
		pub size: u32,
		pub md5: String,
}


impl OriginFile
{
	pub fn new(
			full_path: String,
			path: String,
			size: u32,
			md5: String
			) -> Self
	{
		return OriginFile
		{
			full_path,
			path,
			size,
			md5,
		};
	}
}

