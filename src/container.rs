
//! A container is the outermost part of a Hairball. This provides utilities to
//! read and write into a Container.
//!
//! The format for a Hairball Container is as follows. All words are in
//! little endian order
//!
//! ['hairball'][flags; u32][num segments; u32][uuid [u8; 16]]
//! [first_segment_header; section_header][segment 0; ..N]
//!
//! A segment header is pretty simple
//! [allocated in words; u32][reserved; u32]


use std;
use std::io::Read;
use memmap::{Mmap, Protection};
use uuid::Uuid;
use capnp;

use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};

const MAGIC: [u8; 8] = ['h' as u8, 'a' as u8, 'i' as u8, 'r' as u8,
                        'b' as u8, 'a' as u8, 'l' as u8, 'l' as u8];
const CONTAINER_HEADER_SIZE: u64 = 8 + 4 + 4 + 16;

pub struct Container {
    file: std::fs::File,
    segments: Vec<Segment>,
    uuid: Uuid
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

impl Container {
    /// Internal function to read header of a file returns
    /// the size and flags if the header could be read ans
    /// is valid
    fn read_header<R>(f: &mut R) -> Result<(u32, u32, Uuid), Error>
        where R: std::io::Read
    {
        // Read the magic value to validate the file
        let mut magic = [0; 8];
        if try!(f.read(&mut magic[..])) != 8 {
            return Err(Error::InvalidHeader);
        } else if MAGIC != magic {
            return Err(Error::InvalidHeader);
        }

        let flags = try!(f.read_u32::<LittleEndian>());
        let segments = try!(f.read_u32::<LittleEndian>());

        let mut uuid = [0; 16];
        if try!(f.read(&mut uuid[..])) != 16 {
            return Err(Error::InvalidHeader);
        } 

        Ok((flags, segments, Uuid::from_bytes(&uuid).unwrap()))
    }

    /// Internal function to read header of a file returns
    /// the size and flags if the header could be read ans
    /// is valid
    fn write_header(&mut self) -> Result<(), Error> {
        use std::io::{Write, Seek, SeekFrom};

        try!(self.file.seek(SeekFrom::Start(0)));
        try!(self.file.write(&MAGIC[..]));
        // flags
        try!(self.file.write_u32::<LittleEndian>(0));
        try!(self.file.write_u32::<LittleEndian>(self.segments.len() as u32));
        try!(self.file.write(self.uuid.as_bytes()));

        Ok(())
    }

    /// convert a File handle into a Container
    pub fn read<P>(p: P) -> Result<Container, Error>
        where P: AsRef<std::path::Path>
    {
        let mut f = try!(std::fs::File::open(p));

        let (_, nsegments, uuid) = try!(Container::read_header(&mut f));

        // Get the current offset
        let mut offset = CONTAINER_HEADER_SIZE;
        let mut segments = Vec::with_capacity(nsegments as usize);
        for _ in 0..nsegments {
            let segment = try!(Segment::read(&mut f, offset));
            offset = segment.next_offset();
            segments.push(segment);
        }

        Ok(Container {
            file: f,
            segments: segments,
            uuid: uuid
        })
    }

    /// convert a File handle into a Container
    fn create<P>(p: P) -> Result<Container, Error>
        where P: AsRef<std::path::Path>
    {

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(p);

        let mut c = Container {
            file: try!(file),
            segments: Vec::new(),
            uuid: Uuid::new_v4()
        };

        try!(c.write_header());
        Ok(c)
    }
}

impl capnp::message::ReaderSegments for Container {
    fn get_segment<'a>(&'a self, id: u32) -> Option<&'a [capnp::Word]> {
        let out = self.segments.get(id as usize)
            .map(|seg| seg.words());
        out
    }
}

#[derive(Debug)]
struct SegmentHeader {
    allocated: u32,
    reserved: u32
}

impl SegmentHeader {
    /// read a segment header from a file using the current offset
    fn read(f: &mut std::fs::File) -> Result<SegmentHeader, Error> {
        let allocated = try!(f.read_u32::<LittleEndian>());
        let reserved = try!(f.read_u32::<LittleEndian>());

        Ok(SegmentHeader{
            allocated: allocated,
            reserved: reserved
        })
    }

    /// Write a header into a file with the current offset
    fn write(&self, f: &mut std::fs::File) -> Result<(), std::io::Error> {
        try!(f.write_u32::<LittleEndian>(self.allocated));
        try!(f.write_u32::<LittleEndian>(self.reserved));
        Ok(())
    }
}

struct Segment {
    header: SegmentHeader,
    offset: u64,
    add: usize,
    size: usize,
    map: Mmap
}

fn fix_offset(offset: u64) -> (u64, u64) {
    let page = 4096 as u64;
    let add = (page - 1) & offset;
    (offset - add, add)
}

impl Segment {
    /// Read a segment from a file at a give offset
    fn read(f: &mut std::fs::File, offset: u64) -> Result<Segment, Error> {
        use std::io::{Seek, SeekFrom};

        try!(f.seek(SeekFrom::Start(offset)));

        let header = try!(SegmentHeader::read(f));

        let (offset, add) = fix_offset(offset+8);

        // Memory map the file in RO mode
        let map = try!(Mmap::open_with_offset(f, Protection::Read, offset as usize, header.allocated as usize));

        let size = header.allocated;
        Ok(Segment {
            header: header,
            offset: offset,
            add: add as usize,
            size: size as usize,
            map: map
        })
    }

    /// Creates a new segment for writing
    fn create(f: &mut std::fs::File, offset: u64, size: u32) -> Result<Segment, Error> {
        use std::io::{Write, Seek, SeekFrom};

        try!(f.seek(SeekFrom::Start(offset)));
        let header = SegmentHeader {
            allocated: size,
            reserved: 0
        };
        try!(header.write(f));

        if size != 0 {
            try!(f.seek(SeekFrom::Current(size as i64 - 1)));
            // write an empty byte to create the segment on disk
            try!(f.write(&[0u8]));
        }

        let (offset, add) = fix_offset(offset+8);
        
        // Memory map the file in RO mode
        let map = try!(Mmap::open_with_offset(f, Protection::ReadWrite, offset as usize, size as usize));

        Ok(Segment {
            header: header,
            offset: offset,
            add: add as usize,
            size: size as usize,
            map: map
        })
    }

    /// Used to calculate where the next segment will land
    fn next_offset(&self) -> u64 {
        (self.offset + self.header.allocated as u64 + self.add as u64)
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
        (self.map.ptr() as usize + self.add) as *mut capnp::Word
    }
}

pub struct Builder(Container);

impl Builder {
    pub fn new<P>(p: P) -> Result<Builder, Error>
        where P: AsRef<std::path::Path>
    {
        let c = try!(Container::create(p));
        Ok(Builder(c))
    }

    /*pub fn set_uuid(&mut self, uuid: Uuid) {
        self.0.uuid = uuid;
    }*/
}

impl capnp::message::ReaderSegments for Builder {
    fn get_segment<'a>(&'a self, id: u32) -> Option<&'a [capnp::Word]> {
        let out = self.0.segments.get(id as usize)
            .map(|seg| seg.words());
        out
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        self.0.write_header().unwrap();
    }
}

fn align(addr: u32) -> u32 {
    let page = 4096 as u32;
    let to_align = page - ((page - 1) & addr);
    if to_align == 0 {
        addr
    } else {
        addr + to_align
    }
}

unsafe impl capnp::message::Allocator for Builder {
    fn allocate_segment(&mut self, size: u32) -> (*mut capnp::Word, u32) {
        let offset = if self.0.segments.len() == 0 {
            CONTAINER_HEADER_SIZE
        } else {
            let len = self.0.segments.len();
            self.0.segments[len-1].next_offset()
        };

        let end = align(offset as u32 + size * 8 + 8);
        let size  = end - offset as u32 - 8;
        let segment = Segment::create(&mut self.0.file, offset, size).unwrap();
        let ptr = segment.as_ptr();
        self.0.segments.push(segment);
        (ptr, size / 8)
    }
}
