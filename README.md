# hairball

Hairball is a container format designed around the needs of [whiske-rs](https://github.com/whiske-rs/whiske).
It managers serialization of the entities into a column format. The format of each column is defined
outside of the container and may use either Capn' proto or some custom format if they wish.

The hairball managers only two things for the developer. The Entity naming, converting from the internal
engine entity into a format that is relative to the hairball. This includes foreign entity lookup. Allowing
for one hairball to reference one or more containers in a transparent fashion.

The hairball format itself is designed to make it easy to serialize with minimal buffering. Reading / writing
of the hairball container is done via mmap. You don't need to know the size of the container before you start
serializing, it will map new segments as needed.

Most of the heavy lifting of the format is handed off to Capn' proto. But it does not plan on forcing the user
to user that format for their columns. If your data is better encoded using [`bincode`](https://github.com/TyOverby/bincode) we will have support for it.

## Entities

A Entity in a hairball only contains two pieces of information. An optional name, and an optional parent.
The parents-name create a hierarchical namespace, which can be useful to symbolically link to an entity in another
hairball. Your parent could be a static scene for example. Meaning you can reload different saves that are based
on the same scene without reloading the static scene.

## Goals
 - [x] Zero-Copy read
 - [x] Zero-Copy writes
 - [ ] Entity management across multiple hairballs
 - [ ] Some type of discovery method to find hairballs without explicit paths
 - [ ] Keep it loosely tied to `whiske-rs` (other may want to use it)
 - [ ] Common column types for assets
 - [ ] OpenGEX or Collada -> Hairball converter
