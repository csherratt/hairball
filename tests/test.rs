extern crate uuid;
extern crate hairball;

use hairball::{Reader, Builder, LocalEntity, ExternalEntity};


#[test]
fn write_eid_0_to_10_write() {
    let mut hairball = Builder::new("hairballs/0..10.hairball").unwrap();

    for i in 0..10 {
        hairball.add_entity(
            LocalEntity::named(format!("{}", i))
        );
    }

    hairball.write().unwrap();
}

#[test]
fn write_eid_0_to_10_000_write() {
    let mut hairball = Builder::new("hairballs/0..10_000.hairball").unwrap();

    for i in 0..10_000 {
        hairball.add_entity(
            LocalEntity::named(format!("{}", i))
        );
    }

    hairball.write().unwrap();
}

#[test]
fn read_eid_0_to_10_read() {
    let hairball = Reader::read("hairballs/ref/0..10.hairball").unwrap();
    
    assert_eq!(hairball.entities_len(), 10);
    for i in 0..hairball.entities_len() {
        let e = hairball.entity(i).unwrap();
        assert_eq!(format!("{}", i), e.name().unwrap())
    }
}

#[test]
fn read_eid_0_to_10_000_read() {
    let hairball = Reader::read("hairballs/ref/0..10_000.hairball").unwrap();
    
    assert_eq!(hairball.entities_len(), 10_000);
    for i in 0..hairball.entities_len() {
        let e = hairball.entity(i).unwrap();
        assert_eq!(format!("{}", i), e.name().unwrap())
    }
}

#[test]
fn write_parent_list() {
    let mut hairball = Builder::new("hairballs/parent_list.hairball").unwrap();

    let mut parent = None;
    for _ in 0..10 {
        let e = if let Some(p) = parent {
            LocalEntity::anonymous().parent(p)
        } else {
            LocalEntity::anonymous()
        };
        parent = Some(hairball.add_entity(e));
    }

    hairball.write().unwrap();
}


#[test]
fn read_parent_list() {
    let hairball = Reader::read("hairballs/ref/parent_list.hairball").unwrap();

    assert_eq!(hairball.entities_len(), 10);
    for i in 1..hairball.entities_len() {
        let e = hairball.entity(i).unwrap();
        assert_eq!(Some((i-1) as u32), e.parent());
    }
}

#[test]
fn write_external() {
    let mut hairball = Builder::new("hairballs/external.hairball").unwrap();

    let a = uuid::Uuid::new_v4();
    let b = uuid::Uuid::new_v4();

    for i in 0..5 {
        hairball.add_external_entity(
            ExternalEntity::new(a, format!("{}", i))
        );
        hairball.add_external_entity(
            ExternalEntity::new(b, format!("{}", i))
        );
    }

    hairball.write().unwrap();
}


#[test]
fn read_external() {
    let hairball = Reader::read("hairballs/ref/external.hairball").unwrap();

    assert_eq!(hairball.external_len(), 2);
    let a = hairball.external(0).unwrap();
    let b = hairball.external(1).unwrap();
    assert!(hairball.external(2).is_none());

    assert_eq!(hairball.entities_len(), 10);
    for i in 0..5 {
        assert_eq!(&a, hairball.entity(i*2+0).unwrap().file().unwrap());
        assert_eq!(format!("{}", i), hairball.entity(i*2+0).unwrap().name().unwrap());
        assert_eq!(&b, hairball.entity(i*2+1).unwrap().file().unwrap());
        assert_eq!(format!("{}", i), hairball.entity(i*2+1).unwrap().name().unwrap());
    }
    assert!(hairball.entity(10).is_none());
}

#[test]
fn read_uuid() {
    let hairball = Builder::new("hairballs/uuid.hairball").unwrap();
    let uuid = hairball.uuid();
    hairball.write().unwrap();

    let hairball = Reader::read("hairballs/uuid.hairball").unwrap();
    assert_eq!(uuid, hairball.uuid().unwrap());
}
