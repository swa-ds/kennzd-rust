use std::cmp::Ordering;
use rocket_dyn_templates::serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Kennzeichen {    
    pub kuerzel: String,
    pub stadt: String,
    pub sitz: String,
    pub bundesland: String,
    pub vergabe: String,
    pub neuvergabe: String,
    pub ausgelaufen: bool
}

impl Kennzeichen {
    pub fn empty() -> Kennzeichen {
        Kennzeichen {
            kuerzel: "".to_string(),
            stadt: "".to_string(),
            sitz: "".to_string(),
            bundesland: "".to_string(),
            vergabe: "".to_string(),
            neuvergabe: "".to_string(),
            ausgelaufen: false
        }
    }
    
    pub fn n_a(kuerzel: String) -> Kennzeichen {
        Kennzeichen {
            kuerzel: kuerzel.to_uppercase(),
            stadt: "n/a".to_string(),
            sitz: "n/a".to_string(),
            bundesland: "n/a".to_string(),
            vergabe: "n/a".to_string(),
            neuvergabe: "n/a".to_string(),
            ausgelaufen: false
        }
    }
}

impl Eq for Kennzeichen {}

impl PartialEq<Self> for Kennzeichen {
    fn eq(&self, other: &Self) -> bool {
        self.kuerzel.eq(&other.kuerzel)
    }
}

impl PartialOrd<Self> for Kennzeichen {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.kuerzel.cmp(&other.kuerzel))
    }
}

impl Ord for Kennzeichen {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kuerzel.cmp(&other.kuerzel)
    }
}
