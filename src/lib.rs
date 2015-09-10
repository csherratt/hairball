extern crate capnp;
extern crate uuid;

use std::rc::Rc;
use std::collections::{HashMap, HashSet};

pub mod hairball_capnp {
    include!(concat!(env!("OUT_DIR"), "/hairball_capnp.rs"));
}

const MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
const MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
const PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

pub enum Name {
    Path(String),
    Index(u32)
}

pub enum Entity {
    Local(LocalEntity),
    External(ExternalEntity)
}

impl Entity {
    fn write(&self, mut builder: hairball_capnp::entity::Builder)  {
        match *self {
            Entity::Local(ref e) => {
                e.write(builder.init_local())
            }
            Entity::External(ref e) => {
                e.write(builder.init_external())
            }
        }
    }
}

pub struct LocalEntity {
    name: Option<String>,
    parent: Option<u32>
}

pub struct ExternalEntity {
    file: uuid::Uuid,
    name: Name
}

impl ExternalEntity {
    fn write(&self, mut builder: hairball_capnp::external_entry::Builder) {

    }
}

pub struct HairballBuilder {
    uuid: uuid::Uuid,
    entity: Vec<Rc<Entity>>,
    builder: capnp::message::Builder<capnp::message::HeapAllocator>
}

impl HairballBuilder {
    pub fn new() -> HairballBuilder {
        let mut builder = capnp::message::Builder::new_default();
        builder.init_root::<hairball_capnp::hairball::Builder>();

        HairballBuilder {
            uuid: uuid::Uuid::new_v4(),
            entity: Vec::new(),
            builder: builder
        }
    }

    /// get the uuid of the file
    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    /// set the uuid of the Hairball 
    pub fn set_uuid(&mut self, uuid: uuid::Uuid) {
        self.uuid = uuid;
    }

    /// Adds a local entity to the file's keyspace
    pub fn add_entity(&mut self, entry: LocalEntity) -> u32 {
        self.entity.push(Rc::new(Entity::Local(entry)));
        self.entity.len() as u32 - 1
    }

    /// Adds a external entity to the file's key space
    pub fn add_external_entity(&mut self, entry: ExternalEntity) -> u32 {
        self.entity.push(Rc::new(Entity::External(entry)));
        self.entity.len() as u32 - 1
    }

    fn write_entities(&mut self) {
        let mut root = self.builder.get_root::<hairball_capnp::hairball::Builder>().unwrap();
        let mut entities = root.init_entities(self.entity.len() as u32);
        for (i, e) in self.entity.iter().enumerate() {
            e.write(entities.borrow().get(i as u32));
        }
    }

    fn write_header(&mut self) {
        let mut root = self.builder.get_root::<hairball_capnp::hairball::Builder>().unwrap();
        {
            let mut version = root.borrow().init_version();
            let major: u16 = MAJOR.parse::<u16>().unwrap();
            let minor: u16 = MINOR.parse::<u16>().unwrap();
            let patch: u16 = PATCH.parse::<u16>().unwrap();
            version.set_major(major);
            version.set_minor(minor);
            version.set_patch(patch);
        }
        root.set_uuid(self.uuid.as_bytes());
    }

    /// Write the 
    pub fn write<W>(mut self, w: &mut W)
        where W: std::io::Write
    {
        self.write_header();
        self.write_entities();
        capnp::serialize::write_message(w, &self.builder);
    }
}

impl LocalEntity {
    /// Create a new Entity with the supplied name
    pub fn named(name: String) -> LocalEntity {
        LocalEntity {
            name: Some(name),
            parent: None
        }
    }

    /// Creates an anonymous entity 
    pub fn anonymous() -> LocalEntity {
        LocalEntity {
            name: None,
            parent: None
        }
    }

    /// Set a parent to this object
    pub fn parent(self, parent: u32) -> LocalEntity {
        LocalEntity {
            parent: Some(parent),
            name: self.name,
        }
    }

    fn write(&self, mut builder: hairball_capnp::local_entry::Builder) {
        if let Some(ref name) = self.name {
            builder.set_name(&name[..])
        }
        builder.set_parent(if let Some(id) = self.parent { id } else { !0 });
    }
}
