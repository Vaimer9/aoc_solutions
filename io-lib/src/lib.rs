use std::{
    path::{Path, PathBuf},
    fs::File,
    io::Read
};

enum DataType {
    Raw,
    Trimmed
}

pub struct Sample {
    path: PathBuf,
    raw: String,
    state: DataType
}

impl Sample {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            raw: String::new(),
            state: DataType::Raw
        }
    }
    
    pub fn read(mut self) -> std::io::Result<Self> {
        let mut file = File::open(&self.path)?;
        file.read_to_string(&mut self.raw)?;
        Ok(Self { ..self })
    }

    pub fn trim(self) -> Self {
        Self { state: DataType::Trimmed, ..self }
    }

    pub fn get_raw(&self) -> &str {
        &self.raw
    }

    pub fn get_vec(&self) -> Vec<String> {
        let vec = self
            .get_raw()
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        vec
    }

    pub fn get_trimmed(&self) -> String {
        self.get_raw()
            .chars()
            .filter(|x| *x != '\n')
            .collect()
    }
}

impl From<&Sample> for String {
    fn from(input: &Sample) -> String {
        match input.state {
            DataType::Raw => input.get_raw().to_string(),
            DataType::Trimmed => input.get_trimmed()
        }
    }
}

impl From<&Sample> for Vec<String> {
    fn from(input: &Sample) -> Vec<String> {
        input.get_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::Sample;

    #[test]
    fn raw_string_test() {
        let sample = Sample::new("test.txt")
            .read()
            .unwrap();

        let x = String::from(&sample);
        assert_eq!(&x, "hello\nis there anybody in here\n");
    }

    #[test]
    fn trimmed_test() {
        let sample = Sample::new("test.txt")
            .read().unwrap()
            .trim();

        let x = String::from(&sample);
        assert_eq!(&x, "hellois there anybody in here")
    }

    #[test]
    fn vec_test() {
        let sample = Sample::new("test.txt")
            .read().unwrap();

        let side = Vec::from(&sample);
        assert_eq!(side, vec!["hello", "is there anybody in here"]);
    }

}
