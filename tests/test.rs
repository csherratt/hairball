extern crate hairball;

use hairball::{HairballReader, HairballBuilder, LocalEntity};


#[test]
fn file_eid_0_to_10_write() {
    let mut hairball = HairballBuilder::new();

    for i in 0..10 {
        hairball.add_entity(
            LocalEntity::named(format!("{}", i))
        );
    }

    let mut vec: Vec<u8> = Vec::new();
    hairball.write(&mut vec);
}

#[test]
fn file_eid_0_to_10_read() {
    let mut file = std::fs::File::open("hairballs/ref/0-10.hairball").unwrap();
    let mut hairball = HairballReader::read(&mut file).unwrap();
    
    assert_eq!(hairball.entities_len(), 10);
    for i in 0..hairball.entities_len() {
        let e = hairball.entity(i).unwrap();
        assert_eq!(format!("{}", i), e.name().unwrap())
    }
}