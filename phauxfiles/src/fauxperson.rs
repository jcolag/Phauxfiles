use std::fmt;

#[deriving(Decodable, Encodable)]
pub struct FauxPerson {
    name: String,
    surname: String,
    gender: String,
    country: String,
}

impl fmt::Show for FauxPerson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref name, ref surname) = match self.country.as_slice() {
            // Surnames still come first for Chinese names, presumably
            "China" => (self.surname.clone(), self.name.clone()),
            _ => (self.name.clone(), self.surname.clone()),
        };
        write!(f, "<h2>{} {}</h2>\n{} from <strong>{}</strong>", name, surname, self.gender, self.country)
    }
}

#[deriving(Decodable, Encodable)]
pub struct ImageUrl {
    epic: String,
    bigger: String,
    normal: String,
    mini: String,
}

#[deriving(Decodable, Encodable)]
pub struct FaceCollection {
    username: String,
    image_urls: ImageUrl,
}

impl fmt::Show for FaceCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<img src='{}'>", self.image_urls.epic)
    }
}

