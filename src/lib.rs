extern crate capnp;
extern crate uuid;

use std::collections::{HashMap, HashSet};

pub mod hairball_capnp {
    include!(concat!(env!("OUT_DIR"), "/hairball_capnp.rs"));
}

const MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
const MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
const PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

pub struct HairballBuilder {
    uuid: uuid::Uuid,
    entity: Vec<Entity<String>>,
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
    pub fn add_entity(&mut self, entry: LocalEntity<String>) -> u32 {
        self.entity.push(Entity::Local(entry));
        self.entity.len() as u32 - 1
    }

    /// Adds a external entity to the file's key space
    pub fn add_external_entity(&mut self, entry: ExternalEntity<String>) -> u32 {
        self.entity.push(Entity::External(entry));
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

#[derive(Debug)]
pub struct LocalEntity<T> {
    name: Option<T>,
    parent: Option<u32>
}

impl<T> LocalEntity<T> {
    /// Create a new Entity with the supplied name
    pub fn named(name: T) -> LocalEntity<T> {
        LocalEntity {
            name: Some(name),
            parent: None
        }
    }

    /// Creates an anonymous entity 
    pub fn anonymous() -> LocalEntity<T> {
        LocalEntity {
            name: None,
            parent: None
        }
    }

    /// Set a parent to this object
    pub fn parent(self, parent: u32) -> LocalEntity<T> {
        LocalEntity {
            parent: Some(parent),
            name: self.name,
        }
    }
}

impl LocalEntity<String> {
    fn write(&self, mut builder: hairball_capnp::local_entry::Builder) {
        if let Some(ref name) = self.name {
            builder.set_name(&name[..])
        }
        builder.set_parent(if let Some(id) = self.parent { id } else { !0 });
    }
}

impl<'a> LocalEntity<&'a str> {
    fn read(reader: hairball_capnp::local_entry::Reader<'a>) -> Result<LocalEntity<&'a str>, capnp::Error> {
        Ok(LocalEntity {
            name: if reader.has_name() {
                Some(try!(reader.get_name()))
            } else {
                None
            },
            parent: match reader.get_parent() {
                0xffffffff => None,
                x => Some(x)
            }
        })
    }
}

#[derive(Debug)]
pub enum Entity<T> {
    Local(LocalEntity<T>),
    External(ExternalEntity<T>)
}

impl<'a> Entity<&'a str> {
    pub fn name(&self) -> Option<&str> {
        match *self {
            Entity::Local(ref e) => e.name,
            Entity::External(ref e) => Some(&e.name)
        }
    }

    pub fn parent(&self) -> Option<u32> {
        match *self {
            Entity::Local(ref e) => e.parent,
            Entity::External(_) => None
        }
    }
}

impl Entity<String> {
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

impl<'a> Entity<&'a str> {
    fn read(e: hairball_capnp::entity::Reader<'a>) -> Result<Entity<&'a str>, capnp::Error> {
        use hairball_capnp::entity::Which;

        Ok(match try!(e.which()) {
            Which::Local(l) => {
                Entity::Local(try!(LocalEntity::read(try!(l))))
            }
            Which::External(e) => {
                Entity::External(try!(ExternalEntity::read(try!(e))))
            }
        })
    }
}

#[derive(Debug)]
pub struct ExternalEntity<T> {
    file: uuid::Uuid,
    name: T
}

impl<T> ExternalEntity<T> {
    fn write(&self, mut builder: hairball_capnp::external_entry::Builder) {

    }

    fn read(reader: hairball_capnp::external_entry::Reader) -> Result<ExternalEntity<T>, capnp::Error> {
        panic!()
    }
}

pub struct HairballReader {
    reader: capnp::message::Reader<capnp::serialize::OwnedSegments>,
}

impl HairballReader {
    pub fn read<R>(r: &mut R) -> Result<HairballReader, capnp::Error>
        where R: std::io::Read
    {
        let opts = capnp::message::ReaderOptions::new();
        capnp::serialize::read_message(r, opts)
            .map(|r| HairballReader{reader: r})
    }

    pub fn entities_len(&self) -> usize {
        self.reader.get_root::<hairball_capnp::hairball::Reader>()
            .and_then(|root| root.get_entities()).ok()
            .map(|x| x.len() as usize)
            .unwrap_or(0)
    }

    pub fn entity(&self, idx: usize) -> Option<Entity<&str>> {
        self.reader.get_root::<hairball_capnp::hairball::Reader>()
            .and_then(|root| root.get_entities()).ok()
            .and_then(|entities| {
                if (entities.len() as usize) < idx {
                    None
                } else {
                    Entity::read(entities.get(idx as u32)).ok()
                }
            })
    }
}

