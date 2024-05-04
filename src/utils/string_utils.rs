pub struct StringBuilder
{
    buffer: Vec<char>,
}

impl StringBuilder
{
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self
    {
        Self { buffer: Vec::with_capacity(capacity) }
    }

    pub fn get(&self, index: usize) -> Option<&char>
    {
        return self.buffer.get(index);
    }

    #[allow(dead_code)]
    pub fn get_capacity(&self) -> usize
    {
        return self.buffer.capacity();
    }

    #[allow(dead_code)]
    pub fn empty(&self) -> bool
    {
        return self.buffer.len() == 0;
    }

    #[allow(dead_code)]
    pub fn clear(&mut self)
    {
        self.buffer.clear();
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize
    {
        return self.buffer.len();
    }

    pub fn append_char(&mut self, value: char)
    {
        self.buffer.push(value);
    }

    pub fn append_str(&mut self, value: &str)
    {
        for ch in value.chars()
        {
            self.buffer.push(ch);
        }
    }

    pub fn append_string(&mut self, value: &String)
    {
        for ch in value.chars()
        {
            self.buffer.push(ch);
        }
    }

    #[allow(dead_code)]
    pub fn as_slice(&self) -> &[char]
    {
        return self.buffer.as_slice();
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String
    {
        let mut string = String::with_capacity(self.buffer.len());

        for ch in &self.buffer
        {
            string.push(*ch);
        }

        return string;
    }
}

#[cfg(test)]
mod tests
{
    use crate::utils::string_utils::StringBuilder;

    #[test]
    fn create_string_builder_with_capacity()
    {
        let capacity: usize = 4096;
        let builder = StringBuilder::new(capacity);
        assert_eq!(builder.get_capacity(), capacity);
        assert_eq!(builder.empty(), true);
        assert_eq!(builder.len(), 0);
    }

    #[test]
    fn write_char()
    {
        let capacity: usize = 4096;
        let mut builder = StringBuilder::new(capacity);
        assert_eq!(builder.get_capacity(), capacity);

        builder.append_char('a');
        builder.append_char('b');
        builder.append_char('c');
        let output = String::from("abc");
        assert_eq!(builder.empty(), false);
        assert_eq!(builder.len(), output.len());
        assert_eq!(builder.to_string(), output);
    }

    #[test]
    fn write_hello_world_via_copy()
    {
        let capacity: usize = 4096;
        let mut builder = StringBuilder::new(capacity);
        assert_eq!(builder.get_capacity(), capacity);

        let input = String::from("Hello, world!");
        builder.append_string(&input);
        assert_eq!(builder.empty(), false);
        assert_eq!(builder.len(), input.len());
        assert_eq!(builder.to_string(), input);
    }
}

