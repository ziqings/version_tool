

pub struct ZLText<'a>
{
    content: &'a str,
    lines: Vec<&'a str>,
}

impl<'a> ZLText<'a>
{
    pub fn new(cstr: &'a str) -> ZLText
    {
        let index = cstr.find("\r\n").unwrap_or(0);

        let lines;
        if index > 0
        {
            lines = cstr.split("\r\n").collect();
        }
        else
        {
            lines = cstr.split("\n").collect();
        }

        return ZLText
        {
            content: cstr,
            lines,
        };
    }

    pub fn read(&self, key: &str) -> Vec<&str>
    {
        let mut lst: Vec<&'a str> = Vec::new();

        let mut find: bool = false;

        let tkey = format!("#{}", key);
        for line in &self.lines
        {
            if line.len() < 2
            {
                continue;
            }

            if line.starts_with("#")
            {
                if find
				{
                    break;
				}

                if *line == tkey
				{
                    find = true;
				}
            }

            if !&find
			{
                continue;
			}

            if !line.starts_with("@")
			{
                continue;
			}

            lst.push(&line[1..]);
        }

		return lst;
    }
}

