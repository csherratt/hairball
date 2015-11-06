
//! A container is the outermost part of a Hairball. This provides utilities to
//! read and write into a Container.
//!
//! The format for a Hairball Container is as follows. All words are in
//! little endian order
//!
//! ['hairball'][version; [u32; 3]][flags; u32]
//! [offset: u32][num segments; u32][segment_offset: u64]
//! [uuid; [u8; 16]]
//!
//! A segment header is pretty simple
//! [allocated in words; u32][reserved; u32]


use std;
use std::io::Read;
use memmap::{Mmap, Protection};
use capnp;
use uuid;

use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};

const MAGIC: &'static [u8] = b"hairball";
const CONTAINER_HEADER_SIZE: u64 = 8 + 3 * 4 + 4 + 4 + 4 + 8 + 16;
const DEFAULT_OFFSET: u64 = 4096;
const ALLOC_SIZE: u32 = 4096;

pub struct Container {
    file: std::fs::File,
    segments: Vec<Segment>,
    uuid: uuid::Uuid
}

#[derive(Debug)]
pub enum Error {
    // The header of the file is invalid
    InvalidHeader,
    Io(std::io::Error),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl std::convert::From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        match err {
            byteorder::Error::UnexpectedEOF => Error::InvalidHeader,
            byteorder::Error::Io(e) => Error::Io(e)
        }
    }
}

struct Header {
    version: [u32; 3],
    flags: u32,
    offset: u32,
    num_segments: u32,
    segments_offset: u64,
    uuid: [u8; 16]
}

impl Header {
    /// Read the header form a file doing some basic validation
    fn read<R>(f: &mut R) -> Result<Header, Error>
        where R: std::io::Read
    {
        let mut magic = [0; 8];
        if try!(f.read(&mut magic[..])) != 8 {
            return Err(Error::InvalidHeader);
        } else if MAGIC != magic {
            return Err(Error::InvalidHeader);
        }

        let version = [
            try!(f.read_u32::<LittleEndian>()),
            try!(f.read_u32::<LittleEndian>()),
            try!(f.read_u32::<LittleEndian>()),
        ];

        let flags = try!(f.read_u32::<LittleEndian>());
        let offset = try!(f.read_u32::<LittleEndian>());
        let num_segments = try!(f.read_u32::<LittleEndian>());
        let segments_offset = try!(f.read_u64::<LittleEndian>());
        let mut uuid = [0; 16];
        try!(f.read(&mut uuid));

        Ok(Header{
            offset: offset,
            version: version,
            flags: flags,
            num_segments: num_segments,
            segments_offset: segments_offset,
            uuid: uuid
        })

    }

    /// Write the header to the file
    fn write<W>(&self, f: &mut W) -> Result<(), Error>
        where W: std::io::Write
    {
        try!(f.write(MAGIC));
        for v in &self.version[..] {
            try!(f.write_u32::<LittleEndian>(*v));
        }
        try!(f.write_u32::<LittleEndian>(self.flags));
        try!(f.write_u32::<LittleEndian>(self.offset));
        try!(f.write_u32::<LittleEndian>(self.num_segments));
        try!(f.write_u64::<LittleEndian>(self.segments_offset));
        try!(f.write(&self.uuid[..]));
        Ok(())
    }
}

impl Container {
    /// Internal function to read header of a file returns
    /// the size and flags if the header could be read ans
    /// is valid
    fn write_header(&mut self) -> Result<(), Error> {
        use std::io::{Write, Seek, SeekFrom};

        // We can place the segment table at the start of the file
        // otherwise it gets placed after the last segment
        let offset = if (DEFAULT_OFFSET - CONTAINER_HEADER_SIZE) / 4 > self.segments.len() as u64 {
            CONTAINER_HEADER_SIZE
        } else {
            self.segments[self.segments.len()-1].next_offset()
        };

        // Write out the segment table
        try!(self.file.seek(SeekFrom::Start(offset)));
        for s in &self.segments {
            try!(self.file.write_u32::<LittleEndian>(s.size as u32))
        }

        // turn the uuid in a byte array
        let mut uuid = [0; 16];
        for (i, b) in self.uuid.as_bytes().iter().enumerate() {
            uuid[i] = *b;
        }

        let first = if self.segments.len() == 0 {
            0
        } else {
            self.segments[0].offset as u32
        };

        let version = ::semver::Version::parse(::VERSION).unwrap();

        try!(self.file.seek(SeekFrom::Start(0)));
        Header{
            offset: first,
            version: [version.major as u32,
                      version.minor as u32,
                      version.patch as u32],
            flags: 0,
            num_segments: self.segments.len() as u32,
            segments_offset: offset,
            uuid: uuid
        }.write(&mut self.file)
    }

    /// convert a File handle into a Container
    pub fn read<P>(p: P) -> Result<Container, Error>
        where P: AsRef<std::path::Path>
    {
        use std::io::{Seek, SeekFrom};

        let mut f = try!(std::fs::File::open(p));

        let header = try!(Header::read(&mut f));

        // Seek the segment table then read it
        try!(f.seek(SeekFrom::Start(header.segments_offset)));
        let mut segment_table = Vec::with_capacity(header.num_segments as usize);
        for _ in 0..header.num_segments {
            segment_table.push(
                try!(f.read_u32::<LittleEndian>())
            );
        }

        // Get the current offset
        let mut offset = header.offset as u64;
        let mut segments = Vec::with_capacity(segment_table.len());
        for size in segment_table {
            let s = try!(Segment::read(&mut f, offset, size));
            offset = s.next_offset();
            segments.push(s);
        }


        Ok(Container {
            uuid: uuid::Uuid::from_bytes(&header.uuid[..]).unwrap(),
            file: f,
            segments: segments,
        })
    }

    /// convert a File handle into a Container
    fn create<P>(p: P, uuid: uuid::Uuid) -> Result<Container, Error>
        where P: AsRef<std::path::Path>
    {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(p);

        let mut c = Container {
            uuid: uuid,
            file: try!(file),
            segments: Vec::new(),
        };

        try!(c.write_header());
        Ok(c)
    }

    /// get the uuid of the container
    pub fn uuid(&self) -> uuid::Uuid { self.uuid }
}

impl capnp::message::ReaderSegments for Container {
    fn get_segment<'a>(&'a self, id: u32) -> Option<&'a [capnp::Word]> {
        let out = self.segments.get(id as usize)
            .map(|seg| seg.words());
        out
    }
}

struct Segment {
    offset: u64,
    size: usize,
    map: Mmap 
}

impl Segment {
    /// Read a segment from a file at a give offset
    fn read(f: &mut std::fs::File, offset: u64, size: u32) -> Result<Segment, Error> {
        // Memory map the file in RO mode
        let map = try!(Mmap::open_with_offset(f, Protection::Read, offset as usize, size as usize));

        Ok(Segment {
            offset: offset,
            size: size as usize,
            map: map
        })
    }

    /// Creates a new segment for writing
    fn create(f: &mut std::fs::File, offset: u64, size: u32) -> Result<Segment, Error> {
        use std::io::{Write, Seek, SeekFrom};

        if size != 0 {
            try!(f.seek(SeekFrom::Start(offset + size as u64 - 1)));
            // write an empty byte to create the segment on disk
            try!(f.write(&[0u8]));
        }
        
        // Memory map the file in RO mode
        let map = try!(Mmap::open_with_offset(f, Protection::ReadWrite, offset as usize, size as usize));

        Ok(Segment {
            offset: offset,
            size: size as usize,
            map: map
        })
    }

    /// Used to calculate where the next segment will land
    fn next_offset(&self) -> u64 {
        (self.offset + self.size as u64)
    }


    /// Get the segment as Capn'Protp words
    fn words(&self) -> &[capnp::Word] {
        let ptr = self.as_ptr();
        let len = self.size / 8;

        unsafe {
            std::slice::from_raw_parts(ptr, len)
        }
    }

    fn as_ptr(&self) -> *mut capnp::Word {
        self.map.ptr() as *mut capnp::Word
    }
}

pub struct Builder(Container);

impl Builder {
    pub fn new<P>(p: P, uuid: uuid::Uuid) -> Result<Builder, Error>
        where P: AsRef<std::path::Path>
    {
        let c = try!(Container::create(p, uuid));
        Ok(Builder(c))
    }
}

impl capnp::message::ReaderSegments for Builder {
    fn get_segment<'a>(&'a self, id: u32) -> Option<&'a [capnp::Word]> {
        self.0.segments.get(id as usize).map(|seg| seg.words())
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        self.0.write_header().unwrap();
    }
}

unsafe impl capnp::message::Allocator for Builder {
    fn allocate_segment(&mut self, size: u32) -> (*mut capnp::Word, u32) {
        let offset = if self.0.segments.len() == 0 {
            DEFAULT_OFFSET
        } else {
            let len = self.0.segments.len();
            self.0.segments[len-1].next_offset()
        };

        let size = size * 8;

        // size must be at least ALLOC_SIZE and must also
        // be a multiple of alloc size
        let size = if size < ALLOC_SIZE {
            ALLOC_SIZE
        } else if (ALLOC_SIZE - 1) & size == size {
            size
        } else {
            size + (ALLOC_SIZE - ((ALLOC_SIZE - 1) & size))
        };

        let segment = Segment::create(&mut self.0.file, offset, size).unwrap();
        let ptr = segment.as_ptr();
        self.0.segments.push(segment);
        (ptr, size / 8)
    }
}


pub fn file_uuid<P>(p: P) -> Result<uuid::Uuid, Error>
    where P: AsRef<std::path::Path>
{
    let mut f = try!(std::fs::File::open(p));
    Header::read(&mut f)
        .map(|header| uuid::Uuid::from_bytes(&header.uuid[..]).unwrap())
}

