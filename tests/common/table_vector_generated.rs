// automatically generated by the FlatBuffers compiler, do not modify


pub mod example {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum StatOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Stat<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Stat<'a> {
    type Inner = Stat<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Stat<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Stat {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args StatArgs) -> flatbuffers::WIPOffset<Stat<'bldr>> {
      let mut builder = StatBuilder::new(_fbb);
      builder.add_hp(args.hp);
      builder.finish()
    }

    pub const VT_HP: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn hp(&self) -> u32 {
    self._tab.get::<u32>(Stat::VT_HP, Some(0)).unwrap()
  }
}

pub struct StatArgs {
    pub hp: u32,
}
impl<'a> Default for StatArgs {
    #[inline]
    fn default() -> Self {
        StatArgs {
            hp: 0,
        }
    }
}
pub struct StatBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> StatBuilder<'a, 'b> {
  #[inline]
  pub fn add_hp(&mut self, hp: u32) {
    self.fbb_.push_slot::<u32>(Stat::VT_HP, hp, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> StatBuilder<'a, 'b> {
    let start = _fbb.start_table();
    StatBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Stat<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

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
        args: &'args HeroArgs<'args>) -> flatbuffers::WIPOffset<Hero<'bldr>> {
      let mut builder = HeroBuilder::new(_fbb);
      if let Some(x) = args.stats { builder.add_stats(x); }
      builder.finish()
    }

    pub const VT_STATS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn stats(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Stat<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<Stat<'a>>>>>(Hero::VT_STATS, None)
  }
}

pub struct HeroArgs<'a> {
    pub stats: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<Stat<'a >>>>>,
}
impl<'a> Default for HeroArgs<'a> {
    #[inline]
    fn default() -> Self {
        HeroArgs {
            stats: None,
        }
    }
}
pub struct HeroBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HeroBuilder<'a, 'b> {
  #[inline]
  pub fn add_stats(&mut self, stats: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<Stat<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Hero::VT_STATS, stats);
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

}  // pub mod example

