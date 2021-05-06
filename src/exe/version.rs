


#[derive(RustcDecodable, RustcEncodable)]
pub struct VersionFile
{
path: String,
		  md5: String,
		  zip_size: i32,
		  origin_size: i32,
		  fpt: i32,
		  split_files: Vec<String>,
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
			path,
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
version_file: VersionFile,
				  origin_md5: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct VersionInfo
{
base_files: Vec<VersionFile>,
				extra_files: Vec<VersionFile>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct FullVersion
{
version_files: Vec<FullVersionFile>,
}



pub struct OriginFile
{
	full_path: String,
	path: String,
	size: i32,
	md5: String,
}


