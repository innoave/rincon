
pub trait UseDatabase {
    fn use_database<DbName>(&self, database_name: DbName) -> Self
        where DbName: Into<String>;

    fn use_default_database(&self) -> Self;

    fn database_name(&self) -> Option<&String>;
}

#[derive(Clone, PartialEq, Eq, Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid URL: {}", _0)]
    InvalidUrl(String),
}
