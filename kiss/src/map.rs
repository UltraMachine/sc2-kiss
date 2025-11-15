use super::*;
use bitflags::bitflags;
use std::ops::{Index, IndexMut};

use sc2_prost::ImageData;

use linalg::IVec2;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct BitMap {
	data: Vec<u8>,
	size: IVec2,
}
impl BitMap {
	pub fn new(data: Vec<u8>, size: IVec2) -> Self {
		let is_positive = size.x >= 0 && size.y >= 0;
		assert!(is_positive, "map size can't be negative: got {size}");
		let len_req = (size.x * size.y + 7) as usize / 8;
		let len = data.len();
		assert_eq!(len_req, len, "map size {size} doesn't match data len {len}");
		Self { data, size }
	}
	pub fn size(&self) -> IVec2 {
		self.size
	}

	fn idx(&self, pos: IVec2) -> Option<(usize, usize)> {
		if pos.cmplt(IVec2::ZERO).any() || pos.cmpge(self.size).any() {
			return None;
		}
		let i = (pos.x + pos.y * self.size.x) as usize;
		let byte = i / 8;
		let shift = 7 - i % 8;
		Some((byte, shift))
	}

	pub fn get(&self, pos: IVec2) -> bool {
		let Some((byte, shift)) = self.idx(pos) else {
			return false;
		};
		// SAFETY: pos is checked above to be in bounds
		let b = unsafe { self.data.get_unchecked(byte) };
		b >> shift & 1 != 0
	}
	pub fn set(&mut self, pos: IVec2, val: bool) {
		let Some((byte, shift)) = self.idx(pos) else {
			return;
		};
		// SAFETY: pos is checked above to be in bounds
		let b = unsafe { self.data.get_unchecked_mut(byte) };
		*b = *b & !(1 << shift) | (val as u8) << shift;
	}
}
impl From<ImageData> for BitMap {
	fn from(im: ImageData) -> Self {
		debug_assert_eq!(im.bits_per_pixel, 1);
		Self::new(im.data, im.size.map_or(IVec2::ZERO, Into::into))
	}
}

fn idx_unchecked(pos: IVec2, w: i32) -> usize {
	(pos.x + pos.y * w) as usize
}
fn idx(pos: IVec2, size: IVec2) -> Option<usize> {
	if pos.cmplt(IVec2::ZERO).any() || pos.cmpge(size).any() {
		return None;
	}
	Some(idx_unchecked(pos, size.x))
}
fn pos(i: usize, w: i32) -> IVec2 {
	IVec2::new(i as i32 % w, i as i32 / w)
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Map<T> {
	data: Vec<T>,
	size: IVec2,
}
impl<T> Map<T> {
	pub fn new(data: Vec<T>, size: IVec2) -> Self {
		let is_positive = size.x >= 0 && size.y >= 0;
		assert!(is_positive, "map size can't be negative: got {size}");
		let len_req = (size.x * size.y) as usize;
		let len = data.len();
		assert_eq!(len_req, len, "map size {size} doesn't match data len {len}");
		Self { data, size }
	}
	pub fn size(&self) -> IVec2 {
		self.size
	}

	pub fn get(&self, pos: IVec2) -> Option<&T> {
		self.data.get(idx(pos, self.size)?)
	}
	pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut T> {
		self.data.get_mut(idx(pos, self.size)?)
	}

	/// # Safety
	/// Pos must be in map bounds
	pub unsafe fn get_unchecked(&self, pos: IVec2) -> &T {
		let i = idx_unchecked(pos, self.size.x);
		unsafe { self.data.get_unchecked(i) }
	}
	/// # Safety
	/// Pos must be in map bounds
	pub unsafe fn get_unchecked_mut(&mut self, pos: IVec2) -> &mut T {
		let i = idx_unchecked(pos, self.size.x);
		unsafe { self.data.get_unchecked_mut(i) }
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.data.iter()
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.data.iter_mut()
	}

	pub fn iter_pos(&self) -> impl Iterator<Item = (IVec2, &T)> {
		self.data
			.iter()
			.enumerate()
			.map(|(i, t)| (pos(i, self.size.x), t))
	}
	pub fn iter_mut_pos(&mut self) -> impl Iterator<Item = (IVec2, &mut T)> {
		self.data
			.iter_mut()
			.enumerate()
			.map(|(i, t)| (pos(i, self.size.x), t))
	}
}
impl<T> Index<IVec2> for Map<T> {
	type Output = T;
	fn index(&self, pos: IVec2) -> &Self::Output {
		let size = self.size;
		let i = idx(pos, size).unwrap_or_else(|| panic!("pos {pos} is out of map bounds {size}"));
		// SAFETY: pos is checked above to be in bounds
		unsafe { self.data.get_unchecked(i) }
	}
}
impl<T> IndexMut<IVec2> for Map<T> {
	fn index_mut(&mut self, pos: IVec2) -> &mut Self::Output {
		let size = self.size;
		let i = idx(pos, size).unwrap_or_else(|| panic!("pos {pos} is out of map bounds {size}"));
		// SAFETY: pos is checked above to be in bounds
		unsafe { self.data.get_unchecked_mut(i) }
	}
}

impl From<ImageData> for Map<u8> {
	fn from(im: ImageData) -> Self {
		debug_assert_eq!(im.bits_per_pixel, 8);
		Self::new(im.data, im.size.map_or(IVec2::ZERO, Into::into))
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Height(pub u8);
impl Height {
	pub fn translate(self) -> f32 {
		(self.0 as f32 - 127.) / 8.
	}
}
impl From<u8> for Height {
	fn from(val: u8) -> Self {
		Self(val)
	}
}
impl From<Height> for u8 {
	fn from(h: Height) -> Self {
		h.0
	}
}

pub type HeightMap = Map<Height>;

impl From<ImageData> for HeightMap {
	fn from(im: ImageData) -> Self {
		debug_assert_eq!(im.bits_per_pixel, 8);
		let data = im.data.into_iter().map(Into::into).collect();
		Self::new(data, im.size.map_or(IVec2::ZERO, Into::into))
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Vision {
	Hidden = 0,
	Fog = 1,
	Clear = 2,
	#[default]
	Black,
}
impl Vision {
	pub fn is_clear(self) -> bool {
		self == Self::Clear
	}
}
impl From<u8> for Vision {
	fn from(val: u8) -> Self {
		match val {
			0 => Self::Hidden,
			1 => Self::Fog,
			2 => Self::Clear,
			_ => Self::Black,
		}
	}
}

pub type VisionMap = Map<Vision>;

impl From<ImageData> for VisionMap {
	fn from(im: ImageData) -> Self {
		debug_assert_eq!(im.bits_per_pixel, 8);
		let data = im.data.into_iter().map(Into::into).collect();
		Self::new(data, im.size.map_or(IVec2::ZERO, Into::into))
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
	pub flags: TileFlags,
	pub height: Height,
	pub vision: Vision,
	pub last_seen: u32,
}
bitflags! {
	#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
	#[repr(transparent)]
	pub struct TileFlags: u8 {
		const PATH  = 1;
		const PLACE = 1 << 1;
		const CREEP = 1 << 2;
		// todo: add cliffs, vision blockers, expansion blockers, reserved for buildings or unit paths
	}
}

pub type TileMap = Map<Tile>;

impl TileMap {
	pub fn from_raw(start_raw: sc2_prost::StartRaw, map_state: sc2_prost::MapState) -> Self {
		let size = start_raw.map_size.map_or(IVec2::ZERO, Into::into);

		let pathing = BitMap::from(start_raw.pathing_grid.unwrap_or_default());
		assert_eq!(pathing.size(), size);
		let placement = BitMap::from(start_raw.placement_grid.unwrap_or_default());
		assert_eq!(placement.size(), size);
		let creep = BitMap::from(map_state.creep.unwrap_or_default());
		assert_eq!(creep.size(), size);
		let height = Map::<u8>::from(start_raw.terrain_height.unwrap_or_default());
		assert_eq!(height.size(), size);
		let vision = Map::<u8>::from(map_state.visibility.unwrap_or_default());
		assert_eq!(vision.size(), size);

		let mut map = Self::new(vec![Tile::default(); (size.x * size.y) as usize], size);
		for (pos, tile) in map.iter_mut_pos() {
			if pathing.get(pos) {
				tile.flags.insert(TileFlags::PATH);
			}
			if placement.get(pos) {
				tile.flags.insert(TileFlags::PLACE);
			}
			if creep.get(pos) {
				tile.flags.insert(TileFlags::CREEP);
			}
			tile.height = height[pos].into();
			tile.vision = vision[pos].into();
		}
		map
	}
	// todo: update placement and pathing, or manage it separately from map
	pub fn update(&mut self, map_state: sc2_prost::MapState, game_loop: u32) {
		let creep = BitMap::from(map_state.creep.unwrap_or_default());
		assert_eq!(creep.size(), self.size);
		let vision = Map::<u8>::from(map_state.visibility.unwrap_or_default());
		assert_eq!(vision.size(), self.size);

		for (pos, tile) in self.iter_mut_pos() {
			tile.flags.set(TileFlags::CREEP, creep.get(pos));
			tile.vision = vision[pos].into();
			if tile.vision.is_clear() {
				tile.last_seen = game_loop;
			}
		}
	}
}
