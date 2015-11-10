// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: material.capnp


#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum Component {
  Ambient = 0,
  Diffuse = 1,
  Specular = 2,
}
impl ::capnp::traits::FromU16 for Component {
  #[inline]
  fn from_u16(value : u16) -> ::std::result::Result<Component, ::capnp::NotInSchema> {
    match value {
      0 => ::std::result::Result::Ok(Component::Ambient),
      1 => ::std::result::Result::Ok(Component::Diffuse),
      2 => ::std::result::Result::Ok(Component::Specular),
      n => ::std::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl ::capnp::traits::ToU16 for Component {
  #[inline]
  fn to_u16(self) -> u16 { self as u16 }
}
impl ::capnp::traits::HasTypeId for Component {
  #[inline]
  fn type_id() -> u64 { 0xabf3964faf36e34eu64 }
}

pub mod color {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>
  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>
  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader : reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>
  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> Reader<'a,>
  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_red(self) -> f32 {
      self.reader.get_data_field::<f32>(0)
    }
    #[inline]
    pub fn get_green(self) -> f32 {
      self.reader.get_data_field::<f32>(1)
    }
    #[inline]
    pub fn get_blue(self) -> f32 {
      self.reader.get_data_field::<f32>(2)
    }
    #[inline]
    pub fn get_alpha(self) -> f32 {
      self.reader.get_data_field::<f32>(3)
    }
  }

  pub struct Builder<'a> { builder : ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>
  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>
   {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>
   {
    fn new(builder : ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder : builder,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>
   {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size : u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>
   {
    fn set_pointer_builder<'b>(pointer : ::capnp::private::layout::PointerBuilder<'b>, value : Reader<'a,>) -> Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>
   {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_red(self) -> f32 {
      self.builder.get_data_field::<f32>(0)
    }
    #[inline]
    pub fn set_red(&mut self, value : f32)  {
      self.builder.set_data_field::<f32>(0, value);
    }
    #[inline]
    pub fn get_green(self) -> f32 {
      self.builder.get_data_field::<f32>(1)
    }
    #[inline]
    pub fn set_green(&mut self, value : f32)  {
      self.builder.set_data_field::<f32>(1, value);
    }
    #[inline]
    pub fn get_blue(self) -> f32 {
      self.builder.get_data_field::<f32>(2)
    }
    #[inline]
    pub fn set_blue(&mut self, value : f32)  {
      self.builder.set_data_field::<f32>(2, value);
    }
    #[inline]
    pub fn get_alpha(self) -> f32 {
      self.builder.get_data_field::<f32>(3)
    }
    #[inline]
    pub fn set_alpha(&mut self, value : f32)  {
      self.builder.set_data_field::<f32>(3, value);
    }
  }

  pub struct Pipeline { _typeless : ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless,  }
    }
  }
  impl Pipeline {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 2, pointers : 0 };
    pub const TYPE_ID: u64 = 0xafc2d8185c461115;
  }
}

pub mod binding {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  pub use self::Which::{Texture,Color};

  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>
  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>
  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader : reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>
  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> Reader<'a,>
  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_id(self) -> u32 {
      self.reader.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn get_component(self) -> ::std::result::Result<::material_capnp::Component,::capnp::NotInSchema> {
      ::capnp::traits::FromU16::from_u16(self.reader.get_data_field::<u16>(2))
    }
    pub fn has_color(&self) -> bool {
      if self.reader.get_data_field::<u16>(3) != 1 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(self) -> ::std::result::Result<WhichReader<'a,>, ::capnp::NotInSchema> {
      match self.reader.get_data_field::<u16>(3) {
        0 => {
          return ::std::result::Result::Ok(Texture(
            self.reader.get_data_field::<u32>(2)
          ));
        }
        1 => {
          return ::std::result::Result::Ok(Color(
            ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0))
          ));
        }
        x => return ::std::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Builder<'a> { builder : ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>
  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>
   {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>
   {
    fn new(builder : ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder : builder,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>
   {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size : u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>
   {
    fn set_pointer_builder<'b>(pointer : ::capnp::private::layout::PointerBuilder<'b>, value : Reader<'a,>) -> Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>
   {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_id(self) -> u32 {
      self.builder.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn set_id(&mut self, value : u32)  {
      self.builder.set_data_field::<u32>(0, value);
    }
    #[inline]
    pub fn get_component(self) -> ::std::result::Result<::material_capnp::Component,::capnp::NotInSchema> {
      ::capnp::traits::FromU16::from_u16(self.builder.get_data_field::<u16>(2))
    }
    #[inline]
    pub fn set_component(&mut self, value : ::material_capnp::Component)  {
      self.builder.set_data_field::<u16>(2, value as u16)
    }
    #[inline]
    pub fn set_texture(&mut self, value : u32)  {
      self.builder.set_data_field::<u16>(3, 0);
      self.builder.set_data_field::<u32>(2, value);
    }
    #[inline]
    pub fn set_color<'b>(&mut self, value : ::material_capnp::color::Reader<'b>) -> Result<()> {
      self.builder.set_data_field::<u16>(3, 1);
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value)
    }
    #[inline]
    pub fn init_color(self, ) -> ::material_capnp::color::Builder<'a> {
      self.builder.set_data_field::<u16>(3, 1);
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    pub fn has_color(&self) -> bool {
      if self.builder.get_data_field::<u16>(3) != 1 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(self) -> ::std::result::Result<WhichBuilder<'a,>, ::capnp::NotInSchema> {
      match self.builder.get_data_field::<u16>(3) {
        0 => {
          return ::std::result::Result::Ok(Texture(
            self.builder.get_data_field::<u32>(2)
          ));
        }
        1 => {
          return ::std::result::Result::Ok(Color(
            ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0))
          ));
        }
        x => return ::std::result::Result::Err(::capnp::NotInSchema(x))
      }
    }
  }

  pub struct Pipeline { _typeless : ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless,  }
    }
  }
  impl Pipeline {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 2, pointers : 1 };
    pub const TYPE_ID: u64 = 0x8b4158cfe93b0498;
  }
  pub enum Which<A0> {
    Texture(u32),
    Color(A0),
  }
  pub type WhichReader<'a,> = Which<Result<::material_capnp::color::Reader<'a>>>;
  pub type WhichBuilder<'a,> = Which<Result<::material_capnp::color::Builder<'a>>>;
}

pub mod column {
  #![allow(unused_imports)]
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{text, data, Result};
  use capnp::private::layout;
  use capnp::traits::{FromStructBuilder, FromStructReader};
  use capnp::{primitive_list, enum_list, struct_list, text_list, data_list, list_list};

  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>
  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>
  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader : reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>
  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> Reader<'a,>
  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_bindings(self) -> Result<struct_list::Reader<'a,::material_capnp::binding::Owned<>>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0))
    }
    pub fn has_bindings(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
  }

  pub struct Builder<'a> { builder : ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>
  {
    #[inline]
    fn struct_size() -> layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>
   {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>
   {
    fn new(builder : ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder : builder,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>
   {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size : u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>
   {
    fn set_pointer_builder<'b>(pointer : ::capnp::private::layout::PointerBuilder<'b>, value : Reader<'a,>) -> Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>
   {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_bindings(self) -> Result<struct_list::Builder<'a,::material_capnp::binding::Owned<>>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0))
    }
    #[inline]
    pub fn set_bindings(&mut self, value : struct_list::Reader<'a,::material_capnp::binding::Owned<>>) -> Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(0), value)
    }
    #[inline]
    pub fn init_bindings(self, size : u32) -> struct_list::Builder<'a,::material_capnp::binding::Owned<>> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), size)
    }
    pub fn has_bindings(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
  }

  pub struct Pipeline { _typeless : ::capnp::any_pointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless,  }
    }
  }
  impl Pipeline {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE : layout::StructSize = layout::StructSize { data : 0, pointers : 1 };
    pub const TYPE_ID: u64 = 0xf5ea1843e645f68d;
  }
}