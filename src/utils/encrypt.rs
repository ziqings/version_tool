



pub mod SimpleEncrypt
{
	pub fn encrypt(data: &mut [u8])
	{
		println!("encrypt len->{}", data.len());

		for i in 0 .. data.len()
		{
			data[i] = &data[i] ^ 7;
		}
	}
}


