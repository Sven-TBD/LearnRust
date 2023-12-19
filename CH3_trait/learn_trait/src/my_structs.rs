use crate::my_traits as Info;
pub struct Student {
    name: String,
    age: u32,
}

pub struct Teacher {
    name: String,
    age: u32,
    subject: String,
}

impl Info::GetInformation for Student {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}
impl Info::GetInformation for Teacher {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl Student {
    pub fn new(name: &str, age: u32) -> Student {
        Student { name: name.to_string(), age}
    }
}
impl Teacher {
    pub fn new(name: &str, age: u32,subject: &str) -> Teacher{
        Teacher { name:name.to_string(), age, subject:subject.to_string()}
    }
}

impl Info::SchoolName for Student {}
impl Info::SchoolName for Teacher {}