use std::collections::HashMap;
use crate::model::Kennzeichen;

const KENNZ_DATA: &'static str = include_str!("../resources/kennz.csv");
const BUNDESL_DATA: &'static str = include_str!("../resources/bundesl.csv");

pub struct KennzdRepo {    
    kennzeichen: HashMap<String, Kennzeichen>
}

impl KennzdRepo {
    pub fn find(&self, kuerzel: &str) -> Option<&Kennzeichen> {
        self.kennzeichen.get(&kuerzel.to_uppercase())
    }
    
    pub fn find_all(&self) -> Vec<Kennzeichen> {
        let mut all_kennz: Vec<Kennzeichen> = self.kennzeichen.iter()
            .map(|(_, kennz)| kennz.clone())
            .collect();
        all_kennz.sort();
        all_kennz
    }

    pub fn create() -> KennzdRepo {
        let bundeslaender = read_to_map(BUNDESL_DATA, "bundesl.csv");
        
        let map: HashMap<String, Kennzeichen> = KENNZ_DATA.lines().into_iter()
            .skip(1) // skip header
            .filter(|l| !l.starts_with("~"))
            .map(|l| parse_kennz(l, &bundeslaender))
            .map(|k| (k.kuerzel.clone(), k))
            .collect();
        println!("read {} entries from kennz.csv", map.len());
        println!("Kennzd repo initialized successfully");
        KennzdRepo {
            kennzeichen: map
        }
    }
}

fn read_to_map(file_contents: &str, file_name: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in file_contents.lines() {
        let mut sp = line.split("~");
        let k = sp.next().unwrap().to_string();
        let v = sp.next().unwrap().to_string();
        map.insert(k, v);
    }
    println!("read {} entries from file {}", map.len(), file_name);
    map
}

fn parse_kennz(line: &str, bundeslaender: &HashMap<String, String>) -> Kennzeichen {
    let mut parts = line.split("~");
    let k = parts.next().unwrap();
    let kuerzel = k.replace("*", ""); 
    Kennzeichen {
        kuerzel: kuerzel.clone(),
        stadt: parts.next().unwrap().to_string(),
        sitz: parts.next().unwrap().to_string(),
        bundesland: match bundeslaender.get(parts.next().unwrap()) {
            Some(b) => b.to_string(),
            None => "".to_string(),
        },
        vergabe: parts.next().unwrap().to_string(),
        neuvergabe: parts.next().unwrap().to_string(),
        ausgelaufen: k.contains("*"),
    }
}

#[cfg(test)]
mod tests {
    use crate::repo::KennzdRepo;

    #[test]
    fn test_repo() {
        let r = KennzdRepo::create();
                
        let k = r.find("bc").unwrap();
        assert_eq!("BC", k.kuerzel);
        assert_eq!("Biberach, Kreis", k.stadt);
        assert_eq!("Biberach/Riß", k.sitz);
        assert_eq!("Baden-Württemberg", k.bundesland);
        assert_eq!("seit 1956", k.vergabe);
        assert_eq!("", k.neuvergabe);
        assert_eq!(false, k.ausgelaufen);

        let k = r.find("tt").unwrap();
        assert_eq!("TT", k.kuerzel);
        assert_eq!("ehem. Kreis Tettnang (jetzt FN)", k.stadt);
        assert_eq!("Friedrichshafen", k.sitz);
        assert_eq!("Baden-Württemberg", k.bundesland);
        assert_eq!("1956-1972", k.vergabe);
        assert_eq!("seit 2020", k.neuvergabe);
        assert_eq!(true, k.ausgelaufen);

        let k = r.find("B").unwrap();
        assert_eq!("B", k.kuerzel);
        assert_eq!("Berlin, Bundeshauptstadt", k.stadt);
        assert_eq!("Berlin", k.sitz);
        assert_eq!("Berlin", k.bundesland);
        assert_eq!("seit 1956", k.vergabe);
        assert_eq!("", k.neuvergabe);
        assert_eq!(false, k.ausgelaufen);

        let k = r.find("wg").unwrap();
        assert_eq!("WG", k.kuerzel);
        assert_eq!("ehem. Kreis Wangen/Allgäu (jetzt RV)", k.stadt);
        assert_eq!("Ravensburg", k.sitz);
        assert_eq!("Baden-Württemberg", k.bundesland);
        assert_eq!("1956-1972", k.vergabe);
        assert_eq!("seit 2020", k.neuvergabe);
        assert_eq!(true, k.ausgelaufen);
    }
}
