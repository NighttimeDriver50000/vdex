extern crate enum_repr;
extern crate rand;

pub mod enums;
pub mod veekun;

pub mod abilities;
pub mod items;
pub mod moves;
pub mod natures;
pub mod pokemon;
pub mod types;
pub mod versions;

pub use enums::Enum;
pub use veekun::csv::FromCsv;
use veekun::csv as vcsv;
use veekun::repr::FromVeekun;

#[cfg(test)]
mod tests {
    use abilities;
    use items;
    use moves;
    use natures;
    use pokemon;
    use types;
    use versions;

    use Enum;
    use FromCsv;

    use std::path::Path;

    #[test]
    fn assert_sanity() {
        abilities::assert_sanity();
        items::assert_sanity();
        moves::assert_sanity();
        natures::assert_sanity();
        pokemon::assert_sanity();
        types::assert_sanity();
        versions::assert_sanity();
    }

    fn load_berry_flavors() -> items::berries::BerryFlavorTable {
        let path = Path::new("veekun/berry_flavors.csv");
        items::berries::BerryFlavorTable::from_csv_file(path)
            .expect("Failed to load berry flavor CSV!")
    }

    fn load_berries() -> items::berries::BerryTable {
        let path = Path::new("veekun/berries.csv");
        let mut table = items::berries::BerryTable::from_csv_file(path)
            .expect("Failed to load berry table CSV!");
        table.set_flavors(&load_berry_flavors());
        table
    }

    fn load_palace() -> natures::PalaceTable {
        let path = Path::new("veekun/nature_battle_style_preferences.csv");
        natures::PalaceTable::from_csv_file(path)
            .expect("Failed to load palace table CSV!")
    }

    fn load_efficacy() -> types::EfficacyTable {
        let path = Path::new("veekun/type_efficacy.csv");
        types::EfficacyTable::from_csv_file(path)
            .expect("Failed to load efficacy table CSV!")
    }
    
    #[test]
    fn load_all() {
        load_berries();
        load_palace();
        load_efficacy();
    }

    #[test]
    #[ignore]
    fn print_berries() {
        let table = load_berries();
        for berry in table.table.into_iter() {
            eprintln!("{:?}", berry);
        }
        panic!("Output from this test must be manually inspected.");
    }
    
    #[test]
    #[ignore]
    fn print_palace() {
        let table = load_palace();
        for nature_id in 0..25 {
            let nature = natures::Nature::from_repr(nature_id).unwrap();
            let i = nature_id as usize;
            let high_attack = table.high.attack[i];
            let high_defense = table.high.defense[i];
            let low_attack = table.low.attack[i];
            let low_defense = table.low.defense[i];
            eprintln!("{:?}: high({:?}%, {:?}%), low({:?}%, {:?}%)",
                nature, high_attack, high_defense, low_attack, low_defense);
        }
        panic!("Output from this test must be manually inspected.");
    }

    #[test]
    #[ignore]
    fn print_efficacy() {
        let table = load_efficacy();
        for damage_id in 0..17 {
            let damage = types::Type::from_repr(damage_id).unwrap();
            for target_id in 0..17 {
                let target = types::Type::from_repr(target_id).unwrap();
                let efficacy = table.efficacy(damage, target);
                if efficacy == types::Efficacy::Regular {
                    continue;
                }
                eprintln!("{:?} is {:?} effective against {:?}.",
                    damage, efficacy, target);
            }
        }
        panic!("Output from this test must be manually inspected.");
    }
}
