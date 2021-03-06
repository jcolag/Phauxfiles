use std::fmt;

#[derive(RustcDecodable, RustcEncodable)]
pub struct FauxPerson {
    name: String,
    surname: String,
    gender: String,
    country: String,
}

impl fmt::Display for FauxPerson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref name, ref surname) = match &*self.country {
            // Surnames still come first for Chinese names, presumably
            "China" => (self.surname.clone(), self.name.clone()),
            _ => (self.name.clone(), self.surname.clone()),
        };
        write!(f, "<h2>{} {}</h2>\n{} from <strong>{}</strong>", name, surname, self.gender, self.country)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ImageUrl {
    epic: String,
    bigger: String,
    normal: String,
    mini: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct FaceCollection {
    pub username: String,
    image_urls: ImageUrl,
}

impl fmt::Display for FaceCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<img src='{}'>", self.image_urls.epic)
    }
}

