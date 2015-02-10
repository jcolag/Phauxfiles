pub mod validator {
    fn slice_to_stropt(s: &str) -> Option<String> {
        Some(s.to_string())
    }
    
    pub fn gender(sex: Option<String>) -> Option<String> {
        match sex {
            None => None,
            Some(s) => match s.as_slice() {
                "m" | "M" | "male" => slice_to_stropt("male"),
                "f" | "F" | "female" => slice_to_stropt("female"),
                _ => None,
            }
        }
    }
    
    pub fn country(country: Option<String>) -> Option<String> {
        match country {
            None => None,
            Some(c) => match c.as_slice() {
                "al" => slice_to_stropt("Albania"),
                "ar" => slice_to_stropt("Argentina"),
                "au" => slice_to_stropt("Australia"),
                "at" => slice_to_stropt("Austria"),
                "az" => slice_to_stropt("Azerbaijan"),
                "bd" => slice_to_stropt("Bangladesh"),
                "be" => slice_to_stropt("Belgium"),
                "ba" => slice_to_stropt("Bosnia+and+Herzegovina"),
                "br" => slice_to_stropt("Brazil"),
                "ca" => slice_to_stropt("Canada"),
                "cn" => slice_to_stropt("China"),
                "co" => slice_to_stropt("Colombia"),
                "dk" => slice_to_stropt("Denmark"),
                "eg" => slice_to_stropt("Egypt"),
                "gb" => slice_to_stropt("England"),
                "fi" => slice_to_stropt("Finland"),
                "fr" => slice_to_stropt("France"),
                "ge" => slice_to_stropt("Georgia"),
                "de" => slice_to_stropt("Germany"),
                "gr" => slice_to_stropt("Greece"),
                "hu" => slice_to_stropt("Hungary"),
                "in" => slice_to_stropt("India"),
                "ir" => slice_to_stropt("Iran"),
                "il" => slice_to_stropt("Israel"),
                "it" => slice_to_stropt("Italy"),
                "jp" => slice_to_stropt("Japan"),
                "mx" => slice_to_stropt("Mexico"),
                "ma" => slice_to_stropt("Morocco"),
                "nl" => slice_to_stropt("Netherlands"),
                "nz" => slice_to_stropt("New+Zealand"),
                "ng" => slice_to_stropt("Nigeria"),
                "pl" => slice_to_stropt("Poland"),
                "pt" => slice_to_stropt("Portugal"),
                "ro" => slice_to_stropt("Romania"),
                "ru" => slice_to_stropt("Russia"),
                "es" => slice_to_stropt("Spain"),
                "se" => slice_to_stropt("Sweden"),
                "ch" => slice_to_stropt("Switzerland"),
                "tr" => slice_to_stropt("Turkey"),
                "ua" => slice_to_stropt("Ukraine"),
                "us" => slice_to_stropt("United+States"),
                "vn" => slice_to_stropt("Vietnam"),
                _ => None,
            },
        }
    }
}
