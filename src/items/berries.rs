use std::ffi::OsStr;
use std::path::Path;
use enums::*;
use FromVeekun;
use Type;
use vcsv;
use vcsv::FromCsv;

pub const BERRY_COUNT: usize = 64;

#[EnumRepr(type = "u8")]
pub enum ContestType {
    Cool = 0,
    Tough,
    Cute,
    Beauty,
    Smart,
}

#[EnumRepr(type = "u8")]
pub enum Flavor {
    Spicy = 0,
    Sour,
    Sweet,
    Dry,
    Bitter,
}

impl std::convert::From<Flavor> for ContestType {
    fn from(flavor: Flavor) -> Self {
        ContestType::from_repr(flavor.repr()).unwrap()
    }
}

impl FromVeekun for ContestType {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        match id {
            1 => Some(ContestType::Cool),
            2 => Some(ContestType::Beauty),
            3 => Some(ContestType::Cute),
            4 => Some(ContestType::Smart),
            5 => Some(ContestType::Tough),
            _ => None,
        }
    }
}

impl std::convert::From<ContestType> for Flavor {
    fn from(contest: ContestType) -> Self {
        Flavor::from_repr(contest.repr()).unwrap()
    }
}

impl FromVeekun for Flavor {
    type Intermediate = u8;

    fn from_veekun(id: u8) -> Option<Self> {
        ContestType::from_veekun(id).and_then(|t| Some(Flavor::from(t)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Berry {
    pub item_id: u16,
    pub natural_gift_power: u8,
    pub natural_gift_type: Type,
    pub flavor: Option<Flavor>,
}

impl Default for Berry {
    fn default() -> Self {
        Berry {
            item_id: 0,
            natural_gift_power: 0,
            natural_gift_type: Type::Normal,
            flavor: None,
        }
    }
}

pub struct BerryTable(pub [Berry; BERRY_COUNT]);

impl BerryTable {
    pub fn from_files<'e, S: AsRef<OsStr> + ?Sized>(
        berries_file: &S, flavors_file: &S
    ) -> vcsv::Result<'e, Self> {
        let flavors_path = Path::new(flavors_file);
        let flavors_table = BerryFlavorTable::from_csv_file(flavors_path)?;
        let berries_path = Path::new(berries_file);
        let mut berries_table = BerryTable::from_csv_file(berries_path)?;
        berries_table.set_flavors(&flavors_table);
        Ok(berries_table)
    }

    fn set_flavors(&mut self, flavors: &BerryFlavorTable) {
        for i in 0..BERRY_COUNT {
            let mut max_flavor = None;
            let mut max_value = 0;
            for &flavor in Flavor::VALUES {
                let value = flavors[flavor][i];
                if value > max_value {
                    max_flavor = Some(flavor);
                    max_value = value;
                } else if value == max_value {
                    max_flavor = None;
                }
            }
            self[i].flavor = max_flavor;
        }
    }
}

impl std::ops::Index<usize> for BerryTable {
    type Output = Berry;

    fn index<'a>(&'a self, index: usize) -> &'a Berry {
        self.0.index(index)
    }
}

impl std::ops::IndexMut<usize> for BerryTable {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Berry {
        self.0.index_mut(index)
    }
}

impl vcsv::FromCsvIncremental for BerryTable {
    fn from_empty_csv() -> Self {
        BerryTable([Default::default(); BERRY_COUNT])
    }
    
    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        self[id - 1] = Berry {
            item_id: vcsv::from_field(&record, 1)?,
            natural_gift_power: vcsv::from_field(&record, 3)?,
            natural_gift_type: vcsv::from_field(&record, 4)?,
            flavor: None,
        };
        Ok(())
    }
}

pub struct BerryFlavorTable {
    pub spicy: [u8; BERRY_COUNT],
    pub sour: [u8; BERRY_COUNT],
    pub sweet: [u8; BERRY_COUNT],
    pub dry: [u8; BERRY_COUNT],
    pub bitter: [u8; BERRY_COUNT],
}

impl std::ops::Index<Flavor> for BerryFlavorTable {
    type Output = [u8; BERRY_COUNT];

    fn index<'a>(&'a self, index: Flavor) -> &'a [u8; BERRY_COUNT] {
        match index {
            Flavor::Spicy => &self.spicy,
            Flavor::Sour => &self.sour,
            Flavor::Sweet => &self.sweet,
            Flavor::Dry => &self.dry,
            Flavor::Bitter => &self.bitter,
        }
    }
}

impl std::ops::IndexMut<Flavor> for BerryFlavorTable {
    fn index_mut<'a>(&'a mut self, index: Flavor) -> &'a mut [u8; BERRY_COUNT] {
        match index {
            Flavor::Spicy => &mut self.spicy,
            Flavor::Sour => &mut self.sour,
            Flavor::Sweet => &mut self.sweet,
            Flavor::Dry => &mut self.dry,
            Flavor::Bitter => &mut self.bitter,
        }
    }
}

impl vcsv::FromCsvIncremental for BerryFlavorTable {
    fn from_empty_csv() -> Self {
        BerryFlavorTable {
            spicy: [0; BERRY_COUNT],
            sour: [0; BERRY_COUNT],
            sweet: [0; BERRY_COUNT],
            dry: [0; BERRY_COUNT],
            bitter: [0; BERRY_COUNT],
        }
    }

    fn load_csv_record<'e>(
        &mut self, record: csv::StringRecord
    ) -> vcsv::Result<'e, ()> {
        let id: usize = vcsv::from_field(&record, 0)?;
        let contest_type: ContestType = vcsv::from_field(&record, 1)?;
        let flavor = Flavor::from(contest_type);
        let value = vcsv::from_field(&record, 2)?;
        self[flavor][id - 1] = value;
        Ok(())
    }
}
