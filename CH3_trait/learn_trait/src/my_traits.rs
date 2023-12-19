pub trait GetInformation{
    fn get_name(&self) -> &str;
    fn get_age(&self) -> u32;
}

pub trait SchoolName{
    fn get_school_name(&self) -> String {
        String::from("Hongxing")
    }
}