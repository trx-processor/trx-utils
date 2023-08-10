#[derive(Clone)]
pub struct Base58Address(String);

impl Base58Address {
    pub fn new(address: String) -> Self {
        Self(address)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Into<Base58Address> for String {
    fn into(self) -> Base58Address {
        Base58Address(self)
    }
}

impl<'s> Into<Base58Address> for &'s String {
    fn into(self) -> Base58Address {
        Base58Address(self.to_string())
    }
}

impl<'s> Into<Base58Address> for &'s str {
    fn into(self) -> Base58Address {
        Base58Address(self.to_string())
    }
}
