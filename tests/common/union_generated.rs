// automatically generated by the FlatBuffers compiler, do not modify


pub mod example {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Role {
  NONE = 0,
  Hero = 1,
  Monster = 2,

}

const ENUM_MIN_ROLE: u8 = 0;
const ENUM_MAX_ROLE: u8 = 2;

impl<'a> flatbuffers::Follow<'a> for Role {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Role {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Role;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Role;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Role {
    type Output = Role;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Role>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
const ENUM_VALUES_ROLE:[Role; 3] = [
  Role::NONE,
  Role::Hero,
  Role::Monster
];

#[allow(non_camel_case_types)]
const ENUM_NAMES_ROLE:[&'static str; 3] = [
    "NONE",
    "Hero",
    "Monster"
];

pub fn enum_name_role(e: Role) -> &'static str {
  let index: usize = e as usize;
  ENUM_NAMES_ROLE[index]
}

pub struct RoleUnionTableOffset {}
pub enum HeroOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Hero<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Hero<'a> {
    type Inner = Hero<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Hero<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Hero {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HeroArgs) -> flatbuffers::WIPOffset<Hero<'bldr>> {
      let mut builder = HeroBuilder::new(_fbb);
      builder.add_hp(args.hp);
      builder.finish()
    }

    pub const VT_HP: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn hp(&self) -> u32 {
    self._tab.get::<u32>(Hero::VT_HP, Some(0)).unwrap()
  }
}

pub struct HeroArgs {
    pub hp: u32,
}
impl<'a> Default for HeroArgs {
    #[inline]
    fn default() -> Self {
        HeroArgs {
            hp: 0,
        }
    }
}
pub struct HeroBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HeroBuilder<'a, 'b> {
  #[inline]
  pub fn add_hp(&mut self, hp: u32) {
    self.fbb_.push_slot::<u32>(Hero::VT_HP, hp, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HeroBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HeroBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Hero<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum MonsterOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Monster<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Monster<'a> {
    type Inner = Monster<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Monster<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Monster {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args MonsterArgs) -> flatbuffers::WIPOffset<Monster<'bldr>> {
      let mut builder = MonsterBuilder::new(_fbb);
      builder.add_hp(args.hp);
      builder.finish()
    }

    pub const VT_HP: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn hp(&self) -> u32 {
    self._tab.get::<u32>(Monster::VT_HP, Some(0)).unwrap()
  }
}

pub struct MonsterArgs {
    pub hp: u32,
}
impl<'a> Default for MonsterArgs {
    #[inline]
    fn default() -> Self {
        MonsterArgs {
            hp: 0,
        }
    }
}
pub struct MonsterBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MonsterBuilder<'a, 'b> {
  #[inline]
  pub fn add_hp(&mut self, hp: u32) {
    self.fbb_.push_slot::<u32>(Monster::VT_HP, hp, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MonsterBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MonsterBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Monster<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum PlayerOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Player<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Player<'a> {
    type Inner = Player<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Player<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Player {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args PlayerArgs) -> flatbuffers::WIPOffset<Player<'bldr>> {
      let mut builder = PlayerBuilder::new(_fbb);
      if let Some(x) = args.role { builder.add_role(x); }
      builder.add_role_type(args.role_type);
      builder.finish()
    }

    pub const VT_ROLE_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_ROLE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn role_type(&self) -> Role {
    self._tab.get::<Role>(Player::VT_ROLE_TYPE, Some(Role::NONE)).unwrap()
  }
  #[inline]
  pub fn role(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Player::VT_ROLE, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn role_as_hero(&'a self) -> Option<Hero> {
    if self.role_type() == Role::Hero {
      self.role().map(|u| Hero::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn role_as_monster(&'a self) -> Option<Monster> {
    if self.role_type() == Role::Monster {
      self.role().map(|u| Monster::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct PlayerArgs {
    pub role_type: Role,
    pub role: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for PlayerArgs {
    #[inline]
    fn default() -> Self {
        PlayerArgs {
            role_type: Role::NONE,
            role: None,
        }
    }
}
pub struct PlayerBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PlayerBuilder<'a, 'b> {
  #[inline]
  pub fn add_role_type(&mut self, role_type: Role) {
    self.fbb_.push_slot::<Role>(Player::VT_ROLE_TYPE, role_type, Role::NONE);
  }
  #[inline]
  pub fn add_role(&mut self, role: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Player::VT_ROLE, role);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PlayerBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PlayerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Player<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod example

