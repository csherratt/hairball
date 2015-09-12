extern crate hairball;

use hairball::{HairballReader, HairballBuilder, LocalEntity};


#[test]
fn write_eid_0_to_10_write() {
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
fn read_eid_0_to_10_read() {
    let mut file = std::fs::File::open("hairballs/ref/0-10.hairball").unwrap();
    let hairball = HairballReader::read(&mut file).unwrap();
    
    assert_eq!(hairball.entities_len(), 10);
    for i in 0..hairball.entities_len() {
        let e = hairball.entity(i).unwrap();
        assert_eq!(format!("{}", i), e.name().unwrap())
    }
}

#[test]
fn write_parent_list() {
    let mut hairball = HairballBuilder::new();

    let mut parent = None;
    for i in 0..10 {
        let e = if let Some(p) = parent {
            LocalEntity::anonymous().parent(p)
        } else {
            LocalEntity::anonymous()
        };
        parent = Some(hairball.add_entity(e));
    }

    let mut vec: Vec<u8> = Vec::new();
    hairball.write(&mut vec);
}

#[test]
fn read_parent_list() {
    let mut file = std::fs::File::open("hairballs/ref/parent.hairball").unwrap();
    let hairball = HairballReader::read(&mut file).unwrap();

    assert_eq!(hairball.entities_len(), 10);
    for i in 1..hairball.entities_len() {
        let e = hairball.entity(i).unwrap();
        assert_eq!(Some((i-1) as u32), e.parent());
    }
}