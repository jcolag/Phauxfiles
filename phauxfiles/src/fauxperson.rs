extern crate "rustc-serialize" as rustc_serialize;
use std::fmt;
use std::string;

#[derive(RustcDecodable, RustcEncodable)]
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

impl string::ToString for FauxPerson {
    fn to_string(&self) -> String {
        let (ref name, ref surname) = match self.country.as_slice() {
            // Surnames still come first for Chinese names, presumably
            "China" => (self.surname.clone(), self.name.clone()),
            _ => (self.name.clone(), self.surname.clone()),
        };
        format!("<h2>{} {}</h2>\n{} from <strong>{}</strong>", name, surname, self.gender, self.country)
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
    username: String,
    image_urls: ImageUrl,
}

impl fmt::Show for FaceCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<img src='{}'>", self.image_urls.epic)
    }
}

impl string::ToString for FaceCollection {
    fn to_string(&self) -> String {
        format!("<img src='{}'>", self.image_urls.epic)
    }
}

