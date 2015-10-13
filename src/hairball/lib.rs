extern crate capnp;
extern crate uuid;
extern crate memmap;
extern crate byteorder;
extern crate semver;

use std::collections::HashMap;
pub use container::{Error, file_uuid};

mod container;

#[cfg(feature="build-schema")]
pub mod hairball_capnp {
    include!(concat!(env!("OUT_DIR"), "/hairball_capnp.rs"));
}

#[cfg(not(feature="build-schema"))]
pub mod hairball_capnp;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");


/// A `Builder` is used to construct a hairball
pub struct Builder {
    uuid: uuid::Uuid,
    entity: Vec<Entity<String>>,
    external: Vec<uuid::Uuid>,
    external_lookup: HashMap<uuid::Uuid, u32>,
    builder: capnp::message::Builder<container::Builder>
}

impl Drop for Builder {
    fn drop(&mut self) {
        self.write_entities();
    }
}

impl Builder {
    /// Create a new hairball file with the supplied path. The file will be created
    /// if the file exists it will be truncated.
    pub fn new<P>(p: P) -> Result<Builder, Error>
        where P: AsRef<std::path::Path>
    {
        Builder::new_with_uuid(p, uuid::Uuid::new_v4())
    }

    /// Create a new hairball with a specific uuid
    pub fn new_with_uuid<P>(p: P, uuid: uuid::Uuid) -> Result<Builder, Error>
        where P: AsRef<std::path::Path>
    {
        let mut builder = capnp::message::Builder::new(
            try!(container::Builder::new(p, uuid))
        );
        builder.init_root::<hairball_capnp::hairball::Builder>();

        Ok(Builder {
            uuid: uuid,
            entity: Vec::new(),
            builder: builder,
            external: Vec::new(),
            external_lookup: HashMap::new()
        })
    }

    /// Adds a local entity to the file's keyspace
    pub fn add_entity(&mut self, entry: LocalEntity<String>) -> u32 {
        self.entity.push(Entity::Local(entry));
        self.entity.len() as u32 - 1
    }

    /// Adds a external entity to the file's key space
    pub fn add_external_entity(&mut self, entry: ExternalEntity<String>) -> u32 {
        let insert = self.external_lookup.get(&entry.file).is_none();
        if insert {
            self.external.push(entry.file.clone());
            let id = self.external.len() as u32 - 1;
            self.external_lookup.insert(entry.file.clone(), id);
        }

        self.entity.push(Entity::External(entry));
        self.entity.len() as u32 - 1
    }

    /// internal function that writes the contents of the entities into a file
    fn write_entities(&mut self) {
        let mut root = self.builder.get_root::<hairball_capnp::hairball::Builder>().unwrap();
        {
            let mut entities = root.borrow().init_entities(self.entity.len() as u32);
            for (i, e) in self.entity.iter().enumerate() {
                e.write(entities.borrow().get(i as u32), &self.external_lookup);
            }
        }
        {
            let mut files = root.borrow().init_external(self.external.len() as u32);
            for (i, file) in self.external.iter().enumerate() {
                files.set(i as u32, file.as_bytes());
            }
        }
    }

    /// Write the `metadata` to finalize the hairball
    pub fn close(self) {}

    /// Get the current file uuid
    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    /// Set the uuid file
    pub fn set_uuid(&mut self, uuid: uuid::Uuid) {
        self.uuid = uuid;
    }

    /// access the column that matches this name, iff it does not
    /// exist a column will be created with the provided name
    pub fn column(&mut self, name: &str) -> Result<capnp::any_pointer::Builder, capnp::Error> {
        let root = self.builder.get_root::<hairball_capnp::hairball::Builder>().unwrap();
        let mut column = if root.has_columns() {
            try!(root.get_columns())
        } else {
            root.init_columns()
        };

        loop {
            // found a column that is named ans matches outs
            if column.has_name() && &try!(column.borrow().get_name())[..] == name {
                break;
            // found uninitialized column
            } else if !column.has_name() {
                column.borrow().set_name(name);
                break;
            }

            column = if column.has_next() {
                try!(column.get_next())
            } else {
                column.init_next()
            };
        }

        Ok(if column.has_data() {
            column.get_data()
        } else {
            column.init_data()
        })
    }

    pub fn mapping<E>(&mut self) -> BuilderMapping<E>
        where E: Eq+std::hash::Hash
    {
        BuilderMapping {
            writer: self,
            entities: HashMap::new()
        }
    }
}

pub struct BuilderMapping<'a, E> {
    writer: &'a mut Builder,
    entities: HashMap<E, u32>
}

impl<'a, E> BuilderMapping<'a, E> {
    pub fn entity(&self, e: &E) -> Option<u32>
        where E: Eq + std::hash::Hash
    {
        self.entities.get(e).map(|x| *x)
    }

    pub fn add_entity(&mut self, e: E, local: LocalEntity<String>) -> u32
        where E: Eq + std::hash::Hash
    {
        if let Some(&idx) = self.entities.get(&e) {
            idx
        } else {
            let idx = self.writer.add_entity(local) as u32;
            self.entities.insert(e, idx);
            idx
        }
    }
}

impl<'a, E> std::ops::Deref for BuilderMapping<'a, E> {
    type Target = Builder;
    fn deref(&self) -> &Builder { self.writer }
}

impl<'a, E> std::ops::DerefMut for BuilderMapping<'a, E> {
    fn deref_mut(&mut self) -> &mut Builder { self.writer }
}

#[derive(Clone, Copy, Debug)]
pub struct LocalEntity<T> {
    pub name: Option<T>,
    pub parent: Option<u32>
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

    pub fn file(&self) -> Option<&uuid::Uuid> {
        match *self {
            Entity::Local(_) => None,
            Entity::External(ref e) => Some(&e.file)
        }
    }
}

impl Entity<String> {
    fn write(&self, builder: hairball_capnp::entity::Builder, lookup: &HashMap<uuid::Uuid, u32>) {
        match *self {
            Entity::Local(ref e) => {
                e.write(builder.init_local())
            }
            Entity::External(ref e) => {
                e.write(builder.init_external(), lookup)
            }
        }
    }
}

impl<'a> Entity<&'a str> {
    fn read(e: hairball_capnp::entity::Reader<'a>, root: &Reader) -> Result<Entity<&'a str>, capnp::Error> {
        use hairball_capnp::entity::Which;

        Ok(match try!(e.which()) {
            Which::Local(l) => {
                Entity::Local(try!(LocalEntity::read(try!(l))))
            }
            Which::External(e) => {
                Entity::External(try!(ExternalEntity::read(try!(e), root)))
            }
        })
    }
}

#[derive(Debug)]
pub struct ExternalEntity<T> {
    file: uuid::Uuid,
    name: T
}

impl ExternalEntity<String> {
    pub fn new(file: uuid::Uuid, name: String) -> ExternalEntity<String> {
        ExternalEntity {
            file: file,
            name: name
        }
    }

    fn write(&self, mut builder: hairball_capnp::external_entry::Builder,
             lookup: &HashMap<uuid::Uuid, u32>)
    {
        let file = lookup.get(&self.file).expect("Expected uuid to be in table");
        builder.set_file(*file);
        builder.set_path(&self.name[..]);
    }
}

impl<'a> ExternalEntity<&'a str> {
    fn read(reader: hairball_capnp::external_entry::Reader<'a>,
            root: &Reader) -> Result<ExternalEntity<&'a str>, capnp::Error> {
        let idx = reader.get_file();
        let uuid = root.external(idx as usize).unwrap();
        Ok(ExternalEntity {
            file: uuid,
            name: try!(reader.get_path())
        })
    }
}

pub struct Reader {
    uuid: uuid::Uuid,
    reader: capnp::message::Reader<container::Container>,
}

impl Reader {
    /// Read a `Hairball` from a reader
    pub fn read<P>(p: P) -> Result<Reader, Error>
        where P: AsRef<std::path::Path>
    {
        let mut opts = capnp::message::ReaderOptions::new();
        opts.traversal_limit_in_words = !0;
        opts.nesting_limit = 2_000_000_000;
        container::Container::read(p)
            .map(|r| Reader{
                uuid: r.uuid(),
                reader: capnp::message::Reader::new(r, opts)
            })
    }

    /// Get the number of entities
    pub fn entities_len(&self) -> usize {
        self.reader.get_root::<hairball_capnp::hairball::Reader>()
            .and_then(|root| root.get_entities()).ok()
            .map(|x| x.len() as usize)
            .unwrap_or(0)
    }

    /// Get the entity
    pub fn get_entity(&self, idx: usize) -> Option<Entity<&str>> {
        self.reader.get_root::<hairball_capnp::hairball::Reader>()
            .and_then(|root| Ok(root.get_entities().unwrap())).ok()
            .and_then(|entities| {
                if (entities.len() as usize) <= idx {
                    None
                } else {
                    Some(Entity::read(entities.get(idx as u32), self).unwrap())
                }
            })
    }

    /// Get the number of external references
    pub fn external_len(&self) -> usize {
        self.reader.get_root::<hairball_capnp::hairball::Reader>()
            .and_then(|root| root.get_external()).ok()
            .map(|x| x.len() as usize)
            .unwrap_or(0)
    }

    /// Get an external uuid
    pub fn external(&self, idx: usize) -> Option<uuid::Uuid> {
        self.reader.get_root::<hairball_capnp::hairball::Reader>()
            .and_then(|root| root.get_external()).ok()
            .and_then(|external| {
                if (external.len() as usize) <= idx {
                    None
                } else {
                    external.get(idx as u32).ok()
                }
            })
            .and_then(|x| uuid::Uuid::from_bytes(x))
    }

    /// Get the current file uuid
    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    /// fetch a column with the name, returns None if not column was found
    /// that matches the name
    pub fn column(&self, name: &str) -> Option<capnp::any_pointer::Reader> {
        let root = self.reader.get_root::<hairball_capnp::hairball::Reader>().unwrap();
        let mut column = match root.get_columns() {
            Ok(c) => c,
            Err(_) => return None
        };

        loop {
            // found a column that is named ans matches outs
            match column.borrow().get_name() {
                Err(_) => return None,
                Ok(v) => {
                    if &v[..] == name {
                        break
                    }
                }
            }

            column = match column.get_next() {
                Ok(x) => x,
                Err(_) => return None
            };
        }

        if column.has_data() {
            Some(column.get_data())
        } else {
            None
        }
    }

    /// Create a 
    pub fn into_mapping<E, F>(&self, mut f: F) -> ReaderMapping<E>
        where F: FnMut(usize) -> E
    {
        let e: Vec<E> = (0..self.entities_len()).map(&mut f).collect();

        ReaderMapping {
            reader: self,
            entities: e
        }
    }
}

pub struct ReaderMapping<'a, E> {
    reader: &'a Reader,
    entities: Vec<E>
}

impl<'a, E> ReaderMapping<'a, E> {
    pub fn entity(&self, i: usize) -> Option<&E> {
        self.entities.get(i)
    }
}

impl<'a, E> std::ops::Deref for ReaderMapping<'a, E> {
    type Target = Reader;
    fn deref(&self) -> &Reader { self.reader }
}
