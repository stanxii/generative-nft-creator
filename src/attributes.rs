use rand::thread_rng;
use rand::seq::SliceRandom;

pub const TOTAL_NUMBER_OF_ITEMS: usize = 88;

pub struct AttrDistribution {
    name: String,
    total_number: usize,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    name: String,
    attr_type: AttrType,
    rarity: f32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum AttrType {
    Background,
    Body,
    Face,
}

fn body_types() -> Vec<AttrDistribution> {
    vec![
        AttrDistribution { name: "yellow".to_string(), total_number: 5, },
        AttrDistribution { name: "green".to_string(), total_number: 20, },
        AttrDistribution { name: "normal".to_string(), total_number: 63, },
    ]
}

fn background_types() -> Vec<AttrDistribution> {
    vec![
        AttrDistribution { name: "super_rare".to_string(), total_number: 8, },
        AttrDistribution { name: "black".to_string(), total_number: 40, },
        AttrDistribution { name: "turquoise".to_string(), total_number: 40, },
    ]
}

fn face_types() -> Vec<AttrDistribution> {
    vec![
        AttrDistribution { name: "round".to_string(), total_number: 23, },
        AttrDistribution { name: "square".to_string(), total_number: 27, },
        AttrDistribution { name: "oblong".to_string(), total_number: 38, },
    ]
}

pub fn get_all(attr_type: AttrType) -> Vec<Attribute> {
    let mut return_types = vec![];

    let attr_dists = match attr_type {
        AttrType::Background => background_types(),
        AttrType::Body => body_types(),
        AttrType::Face => face_types(),
    };

    for attr_dist in attr_dists {
        // need to cast to f32
        let num_of_attrs: f32 = attr_dist.total_number as f32;
        let total_num: f32 = TOTAL_NUMBER_OF_ITEMS as f32; 
        // setup attribute
        let attribute = Attribute { 
            name: attr_dist.name,
            attr_type: attr_type.clone(),
            rarity: num_of_attrs / total_num,
        };
        return_types.extend(vec![attribute; attr_dist.total_number]);
    }
    // make sure to shuffle before returning
    // https://docs.rs/rand/0.6.4/rand/seq/trait.SliceRandom.html
    return_types.shuffle(&mut thread_rng());
    return_types
}