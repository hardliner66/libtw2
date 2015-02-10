#![cfg(not(test))]

#![feature(globs)]

extern crate datafile;

use std::fmt::Show;
use std::io::File;

use datafile::Datafile;
use datafile::DatafileItem;
use datafile::DatafileReader;

use internals::*;

pub mod internals;

fn main() {
    let file = box File::open(&Path::new("../dm1.map")).unwrap();
    let dfr = DatafileReader::read(file).unwrap().unwrap();

    for item in dfr.items() {
        let item: DatafileItem = item;
        fn print_map_item<T:MapItem+Show>(slice: &[i32]) {
            let mi: Option<&T> = MapItemExt::from_slice(slice);
            print!("{} ", mi);
        }
        match item.type_id {
            0 => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemVersionV1>(item.data);
                println!("");
            },
            1 => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemInfoV1>(item.data);
                println!("");
            },
            2 => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemImageV1>(item.data);
                print_map_item::<MapItemImageV2>(item.data);
                println!("");
            },
            3 => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemEnvelopeV1>(item.data);
                print_map_item::<MapItemEnvelopeV2>(item.data);
                println!("");
            },
            4 => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemGroupV1>(item.data);
                print_map_item::<MapItemGroupV2>(item.data);
                println!("");
            },
            5 => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemLayerV1>(item.data);
                println!("");
            },
            6 => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                print_map_item::<MapItemEnvpointsV1>(item.data);
                println!("");
            },
            _ => {
                print!("{} {} {} ", item.type_id, item.id, item.data.len());
                print_map_item::<MapItemCommonV0>(item.data);
                panic!("unknown datafile item type");
            },
        }
        //println!("{}", item);
    }
}