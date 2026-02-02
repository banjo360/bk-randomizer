use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::cmp;
use std::error::Error;
use std::io::Read;
use std::io::Write;

mod camera;
pub use camera::Camera;
mod cube;
pub use cube::Cube;
mod lighting;
pub use lighting::Lighting;
mod prop_1;
pub use prop_1::Prop1;
mod prop_2;
pub use prop_2::Prop2;

pub struct MapSetup {
    pub cubes: Vec<Cube>,
    pub cameras: Vec<Camera>,
    pub lightings: Vec<Lighting>,
}

impl MapSetup {
    pub fn new<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let header = reader.read_u16::<BigEndian>()?;
        assert_eq!(header, 0x0101);
        let negative_x_cube_count = reader.read_i32::<BigEndian>()?;
        let negative_y_cube_count = reader.read_i32::<BigEndian>()?;
        let negative_z_cube_count = reader.read_i32::<BigEndian>()?;
        let positive_x_cube_count = reader.read_i32::<BigEndian>()?;
        let positive_y_cube_count = reader.read_i32::<BigEndian>()?;
        let positive_z_cube_count = reader.read_i32::<BigEndian>()?;

        let mut cubes = vec![];
        for x in negative_x_cube_count..=positive_x_cube_count {
            for y in negative_y_cube_count..=positive_y_cube_count {
                for z in negative_z_cube_count..=positive_z_cube_count {
                    let mut cube = Cube::new(reader)?;
                    cube.x = x;
                    cube.y = y;
                    cube.z = z;

                    cubes.push(cube);
                }
            }
        }

        let end = reader.read_u8()?;
        assert_eq!(end, 0);

        let camera_header = reader.read_u8()?;
        assert_eq!(camera_header, 3);

        let mut cameras = vec![];

        let mut camera_header = reader.read_u8()?;
        while camera_header == 1 {
            cameras.push(Camera::new(reader)?);

            camera_header = reader.read_u8()?;
        }
        assert_eq!(camera_header, 0);

        let lighting_header = reader.read_u8()?;
        assert_eq!(lighting_header, 4);

        let mut lightings = vec![];

        let mut section_id = reader.read_u8()?;
        while section_id == 1 {
            lightings.push(Lighting::new(reader)?);

            section_id = reader.read_u8()?;
        }
        assert_eq!(section_id, 0);

        let end = reader.read_u8()?;
        assert_eq!(end, 0);

        Ok(Self {
            cubes,
            cameras,
            lightings,
        })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        let mut negative_x_cube_count = i32::MAX;
        let mut negative_y_cube_count = i32::MAX;
        let mut negative_z_cube_count = i32::MAX;
        let mut positive_x_cube_count = i32::MIN;
        let mut positive_y_cube_count = i32::MIN;
        let mut positive_z_cube_count = i32::MIN;

        for c in &self.cubes {
            negative_x_cube_count = cmp::min(c.x, negative_x_cube_count);
            positive_x_cube_count = cmp::max(c.x, positive_x_cube_count);
            negative_y_cube_count = cmp::min(c.y, negative_y_cube_count);
            positive_y_cube_count = cmp::max(c.y, positive_y_cube_count);
            negative_z_cube_count = cmp::min(c.z, negative_z_cube_count);
            positive_z_cube_count = cmp::max(c.z, positive_z_cube_count);
        }

        writer.write_u16::<BigEndian>(0x0101)?;
        writer.write_i32::<BigEndian>(negative_x_cube_count)?;
        writer.write_i32::<BigEndian>(negative_y_cube_count)?;
        writer.write_i32::<BigEndian>(negative_z_cube_count)?;
        writer.write_i32::<BigEndian>(positive_x_cube_count)?;
        writer.write_i32::<BigEndian>(positive_y_cube_count)?;
        writer.write_i32::<BigEndian>(positive_z_cube_count)?;

        for c in &self.cubes {
            c.write(writer)?;
        }

        writer.write_u8(0)?;
        writer.write_u8(3)?;

        for cam in &self.cameras {
            writer.write_u8(1)?;
            cam.write(writer)?;
        }

        writer.write_u8(0)?;
        writer.write_u8(4)?;

        for light in &self.lightings {
            light.write(writer)?;
        }

        writer.write_u8(0)?;
        writer.write_u8(0)?;

        Ok(())
    }
}
