// use crate::utils::align_reader;
// use crate::utils::read_2_i16;
// use crate::utils::read_3_i16;
// use byteorder::BigEndian;
// use byteorder::ReadBytesExt;
// use byteorder::WriteBytesExt;
// use geometry_layout::GeometryLayout;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::Write;

pub mod geometry_layout;

pub struct Model {
    buffer: Vec<u8>,
}

impl Model {
    pub fn new<R: Read + Seek>(reader: &mut R, length: usize) -> Result<Self, Box<dyn Error>> {
        // let start = reader.seek(std::io::SeekFrom::Current(0))?;

        // println!("\n{start:#X}");

        // let magic = reader.read_u32::<BigEndian>()?;
        // assert_eq!(magic, 0xB);
        // let geometry_layout_offset = reader.read_u32::<BigEndian>()?;
        // let texture_setup_offset = reader.read_u16::<BigEndian>()?;
        // assert_eq!(texture_setup_offset, 0x38);
        // let geo_type = reader.read_u16::<BigEndian>()?;
        // let display_list_setup_offset = reader.read_u32::<BigEndian>()?;
        // let vertex_store_setup_offset = reader.read_u32::<BigEndian>()?;
        // let unknown = reader.read_u32::<BigEndian>()?;
        // let animation_setup_offset = reader.read_u32::<BigEndian>()?;
        // let collision_setup_offset = reader.read_u32::<BigEndian>()?;
        // let effects_setup_end_address = reader.read_u32::<BigEndian>()?;
        // let effects_setup = reader.read_u32::<BigEndian>()?;
        // let unknown2 = reader.read_u32::<BigEndian>()?;
        // let animated_textures_offset = reader.read_u32::<BigEndian>()?;
        // let tri_count = reader.read_u16::<BigEndian>()?;
        // let vert_count = reader.read_u16::<BigEndian>()?;
        // let unknown3 = reader.read_f32::<BigEndian>()?;

        // println!("geometry_layout_offset: {geometry_layout_offset:X}");
        // println!("geo_type: {geo_type:X}");
        // println!("display_list_setup_offset: {display_list_setup_offset:X}");
        // println!("vertex_store_setup_offset: {vertex_store_setup_offset:X}");
        // println!("unknown: {unknown:X}");
        // println!("animation_setup_offset: {animation_setup_offset:X}");
        // println!("collision_setup_offset: {collision_setup_offset:X}");
        // println!("effects_setup_end_address: {effects_setup_end_address:X}");
        // println!("effects_setup: {effects_setup:X}");
        // println!("unknown2: {unknown2:X}");
        // println!("animated_textures_offset: {animated_textures_offset:X}");
        // println!("tri_count: {tri_count}");
        // println!("vert_count: {vert_count}");
        // println!("unknown3: {unknown3}");

        // // texture setup
        // let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        // assert_eq!(start + texture_setup_offset as u64, pos);

        // let size = reader.read_u32::<BigEndian>()?;
        // let texture_count = reader.read_u16::<BigEndian>()?;
        // let padding = reader.read_u16::<BigEndian>()?;
        // assert_eq!(padding, 0);
        // println!("{size:X}");
        // println!("{texture_count} textures");
        // assert_eq!(8 + 0x10 * texture_count as u32, size);

        // for _ in 0..texture_count {
        //     let offset = reader.read_u32::<BigEndian>()?;
        //     let kind = reader.read_u16::<BigEndian>()?;
        //     let padding = reader.read_u16::<BigEndian>()?;
        //     assert_eq!(padding, 0);
        //     let width = reader.read_u8()?;
        //     let height = reader.read_u8()?;
        //     let unk1 = reader.read_u16::<BigEndian>()?; // texture id?
        //     let unk2 = reader.read_u32::<BigEndian>()?;
        //     println!("{offset} for {kind} ({unk1}, {unk2}), {width} x {height}");
        // }

        // // display list setup
        // let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        // assert_eq!(start + display_list_setup_offset as u64, pos);

        // let command_count = reader.read_u32::<BigEndian>()?;
        // println!("{command_count} command_count");
        // let padding = reader.read_u32::<BigEndian>()?;
        // if padding != 0 {
        //     println!("{padding}!!! padding");
        // }
        // for _ in 0..command_count {
        //     let _command = reader.read_u64::<BigEndian>()?;
        // }

        // // vertex store setup
        // let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        // assert_eq!(start + vertex_store_setup_offset as u64, pos);

        // let _min = read_3_i16(reader)?;
        // let _max = read_3_i16(reader)?;
        // let _centre = read_3_i16(reader)?;
        // let _largest_distance_to_centre = reader.read_i16::<BigEndian>()?;
        // let vertex_count = reader.read_u16::<BigEndian>()?;
        // let _largest_distance_to_origin = reader.read_i16::<BigEndian>()?;
        // assert_eq!(vert_count, vertex_count);

        // for _ in 0..vertex_count {
        //     let _part1 = reader.read_u64::<BigEndian>()?;
        //     let _part2 = reader.read_u64::<BigEndian>()?;
        // }

        // if unknown != 0 {
        //     align_reader(reader)?;
        //     let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        //     println!("unknown {:X}, {unknown:X}", pos - start);
        //     assert_eq!(start + unknown as u64, pos);

        //     let mut buffer = vec![0u8; 0x18];
        //     reader.read(&mut buffer)?;
        //     println!("{buffer:?}");
        // }

        // // collision setup
        // if collision_setup_offset != 0 {
        //     align_reader(reader)?;
        //     let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        //     println!(
        //         "collision_setup_offset {:X}, {collision_setup_offset:X}",
        //         pos - start
        //     );
        //     assert_eq!(start + collision_setup_offset as u64, pos);

        //     let _min = read_3_i16(reader)?;
        //     let _max = read_3_i16(reader)?;
        //     let _stride = read_2_i16(reader)?;

        //     let geo_count = reader.read_u16::<BigEndian>()?;
        //     let _scale = reader.read_u16::<BigEndian>()?;
        //     let tri_count = reader.read_u16::<BigEndian>()?;
        //     let padding = reader.read_u16::<BigEndian>()?;
        //     assert_eq!(padding, 0);

        //     for _ in 0..geo_count {
        //         let _start_tri_index = reader.read_u16::<BigEndian>()?;
        //         let _tri_count = reader.read_u16::<BigEndian>()?;
        //     }

        //     for _ in 0..tri_count {
        //         let _vtx_indx_1 = reader.read_u16::<BigEndian>()?;
        //         let _vtx_indx_2 = reader.read_u16::<BigEndian>()?;
        //         let _vtx_indx_3 = reader.read_u16::<BigEndian>()?;
        //         let _unk = reader.read_u16::<BigEndian>()?;
        //         let _flags = reader.read_u32::<BigEndian>()?;
        //     }
        // }

        // // geometry layout
        // let before_pos = reader.seek(std::io::SeekFrom::Current(0))?;
        // align_reader(reader)?;
        // let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        // println!(
        //     "geometry_layout_offset {:X}, {:X}, {geometry_layout_offset:X}",
        //     before_pos - start,
        //     pos - start
        // );
        // assert_eq!(start + geometry_layout_offset as u64, pos);

        // loop {
        //     let pos = reader.seek(std::io::SeekFrom::Current(0))?;
        //     println!("- {pos:X}");

        //     let (_geo_layout, last) = GeometryLayout::new(reader)?;
        //     println!("layout is last? {last}");

        //     if last {
        //         break;
        //     }
        // }

        // todo!();
        let mut buffer = vec![0u8; length];
        reader.read(&mut buffer)?;
        Ok(Self { buffer })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write(&self.buffer)?;
        Ok(())
    }
}
