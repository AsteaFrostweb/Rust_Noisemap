
use std::{clone, fs::File, io::{Write, Read}, path::Path};

use libnoise::prelude::*;
use bevy::{prelude::*, math::f32, render::{texture::ImageSampler, render_resource::{TextureDescriptor, TextureDimension, TextureFormat, Extent3d, TextureUsages}}};



//-------------------------------------------------------------------Defining Noise Values-------------------------------------------------------------------//
#[derive(Clone)]
pub struct NoiseValues
{
    pub octaves: usize,
    pub scale: f32,
    pub persistence: f32,
    pub lacunarity: f32
}


//-------------------------------------------------------------------Defining Noise Source 2D-------------------------------------------------------------------//
trait NoiseSource2D {
    fn sample(&self, point: [f64; 2]) -> f64;
}

impl NoiseSource2D for Perlin<2> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        return libnoise::Generator::sample(self, point);
    }
}

impl NoiseSource2D for Simplex<2> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        return libnoise::Generator::sample(self, point);
    }
}

impl NoiseSource2D for Worley<2> {
    fn sample(&self, point: [f64; 2]) -> f64 {
        return libnoise::Generator::sample(self, point);
    }
}


//-------------------------------------------------------------------Defining Noise Generator 2D-------------------------------------------------------------------//
pub struct NoiseGenerator2D<S>
where
    S: NoiseSource2D,
{
    pub source: S,
    pub values: NoiseValues,
}

impl<S> NoiseGenerator2D<S>
where
    S: NoiseSource2D,
{
    pub fn get_value_at(&self, x: i32, y: i32) -> f32 {
        let mut sum = 0.0;
        let mut amplitude: f32 = 1.0;
        let mut frequency = self.values.scale;

        for _ in 0..self.values.octaves {
            let new_x: f32 = x as f32 * frequency;
            let new_y: f32 = y as f32 * frequency;
            let sample: f64 = self.source.sample([new_x as f64, new_y as f64]);
            sum += sample as f32 * amplitude;

            amplitude *= self.values.persistence;
            frequency *= self.values.lacunarity;
        }

        sum
    }
}


//-------------------------------------------------------------------Defining Combined Nosie Generator 2D-------------------------------------------------------------------//
pub struct CombinedNoiseGenerator2D
{
    perlin_generator: NoiseGenerator2D<Perlin<2>>,
    simplex_generator: NoiseGenerator2D<Simplex<2>>,
    worley_generator: NoiseGenerator2D<Worley<2>>
}
impl CombinedNoiseGenerator2D
{
    pub fn get_combined_value_at_weighted(&self, x: i32, y: i32, perlin_weight: f32, simplex_weight: f32, worley_weight: f32) -> f32
    {
        let perlin = self.perlin_generator.get_value_at(x, y) * perlin_weight;
        let simplex = self.simplex_generator.get_value_at(x, y) * simplex_weight;
        let worley = self.worley_generator.get_value_at(x, y) * worley_weight;
        let ratio = 1.0 / (perlin_weight + simplex_weight + worley_weight);
        return perlin * ratio + simplex * ratio + worley * ratio;
    }
}


//-------------------------------------------------------------------Defining Noise Source 3D-------------------------------------------------------------------//
trait NoiseSource3D {
    fn sample(&self, point: [f64; 3]) -> f64;
}

impl NoiseSource3D for Perlin<3> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        return libnoise::Generator::sample(self, point);
    }
}

impl NoiseSource3D for Simplex<3> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        return libnoise::Generator::sample(self, point);
    }
}

impl NoiseSource3D for Worley<3> {
    fn sample(&self, point: [f64; 3]) -> f64 {
        return libnoise::Generator::sample(self, point);
    }
}


//-------------------------------------------------------------------Defining Noise Generator 3D-------------------------------------------------------------------//

pub struct NoiseGenerator3D<S>
where
    S: NoiseSource3D,
{
    pub source: S,
    pub values: NoiseValues,
}

impl<S> NoiseGenerator3D<S>
where
    S: NoiseSource3D,
{
    pub fn get_value_at(&self, x: i32, y: i32, z: i32) -> f32 {
        let mut sum = 0.0;
        let mut amplitude: f32 = 1.0;
        let mut frequency = self.values.scale;

        for _ in 0..self.values.octaves {
            let new_x: f32 = x as f32 * frequency;
            let new_y: f32 = y as f32 * frequency;
            let new_z: f32 = z as f32 * frequency;
            let sample: f64 = self.source.sample([new_x as f64, new_y as f64, new_z as f64]);
            sum += sample as f32 * amplitude;

            amplitude *= self.values.persistence;
            frequency *= self.values.lacunarity;
        }

        sum
    }
}


//-------------------------------------------------------------------Defining Combined Nosie Generator 3D-------------------------------------------------------------------//
pub struct CombinedNoiseGenerator3D
{
    perlin_generator: NoiseGenerator3D<Perlin<3>>,
    simplex_generator: NoiseGenerator3D<Simplex<3>>,
    worley_generator: NoiseGenerator3D<Worley<3>>
}
impl CombinedNoiseGenerator3D
{
    pub fn get_combined_value_at_weighted(&self, x: i32, y: i32, z: i32, perlin_weight: f32, simplex_weight: f32, worley_weight: f32) -> f32
    {
        let perlin = self.perlin_generator.get_value_at(x, y, z) * perlin_weight;
        let simplex = self.simplex_generator.get_value_at(x, y, z) * simplex_weight;
        let worley = self.worley_generator.get_value_at(x, y, z) * worley_weight;
        let ratio = 1.0 / (perlin_weight + simplex_weight + worley_weight);
        return perlin * ratio + simplex * ratio + worley * ratio;
    }
}


//-------------------------------------------------------------------Defining Noise Map 2D-------------------------------------------------------------------//
pub struct NoiseMap2D
{
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<f32>
} 
impl NoiseMap2D
{   
    fn initialize(&mut self)
    {
        self.buffer = Vec::with_capacity(self.width * self.height)        
    }
    fn get_value_at(&self, x: u32, y: u32) -> f32
    {
        return self.buffer[((y as usize * self.height) + x as usize)]
    }
    fn set_value_at(&mut self, x: u32, y: u32, value: f32)
    {
        self.buffer[((y as usize * self.height) + x as usize)] = value;
    }
    fn to_image(&self) -> Image {
        let mut image = Image {
            data: Vec::new(),
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d{width: self.width as u32, height: self.height as u32, depth_or_array_layers: 1},
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::all(),
                view_formats: &[TextureFormat::Rgba8UnormSrgb],                
            },
            sampler: ImageSampler::default(),
            texture_view_descriptor: None,
        };

        for &value in &self.buffer {
            let pixel_value = (value * 255.0).round() as u8;
            image.data.push(pixel_value);
            image.data.push(pixel_value);
            image.data.push(pixel_value);
            image.data.push(255); // Alpha channel
        }

        image
    }
    
    fn save(&self, path: String) -> Result<bool, std::io::Error> //returns true if sucessfuly saved and false if not
    {

        if !Path::new(&path).extension().map_or(false, |ext| ext == "noise2d") {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file extension"));
        }

        //format of .noise2d file // 8 Bytes "width", 8 Bytes "height", each proceeding 4 Byte chunk is a f32 starting from the 0 position in the "buffer"
        let mut file = File::create(path)?;
        
        // Write dimensions to the file
        file.write_all(&(self.width as u64).to_ne_bytes())?;
        file.write_all(&(self.height as u64).to_ne_bytes())?;        

        // Write buffer data to the file
        for &value in &self.buffer {
            file.write_all(&value.to_ne_bytes())?;
        }

        Ok(true)
    }
    
    fn load(path: &str) -> Result<NoiseMap2D, std::io::Error> {

        if !Path::new(&path).extension().map_or(false, |ext| ext == "noise2d") {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file extension"));
        }

        let mut file = File::open(path)?;

        // Read dimensions from the file
        let mut width_bytes = [0u8; 8];
        let mut height_bytes = [0u8; 8];
      

        file.read_exact(&mut width_bytes)?;
        file.read_exact(&mut height_bytes)?;
        

        let width = u64::from_ne_bytes(width_bytes) as usize;
        let height = u64::from_ne_bytes(height_bytes) as usize;
      

        // Read buffer data from the file
        let mut buffer = vec![0.0; width * height];
        for value in &mut buffer {
            let mut value_bytes = [0u8; 4];
            file.read_exact(&mut value_bytes)?;
            *value = f32::from_ne_bytes(value_bytes);
        }

        Ok(NoiseMap2D { width, height, buffer })
    }

}





//-------------------------------------------------------------------Defining Noise Map 3D-------------------------------------------------------------------//
pub struct NoiseMap3D
{
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub buffer: Vec<f32>
}
impl NoiseMap3D
{
    fn initialize(&mut self)
    {
        self.buffer = vec![0.0; self.width * self.height * self.depth];        
    }

    fn get_value_at(&self, x: u32, y: u32, z: u32) -> f32
    {
        return self.buffer[(z as usize * self.height * self.width) + (y as usize * self.height) + x as usize]
    }

    fn set_value_at(&mut self, x: u32, y: u32, z: u32, value: f32)
    {   
        self.buffer[(z as usize * self.height * self.width) + (y as usize * self.height) + x as usize] = value;
    }

    fn to_image(&self) -> Image {
        let mut image = Image {
            data: Vec::new(),
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: self.width as u32,
                    height: self.height as u32,
                    depth_or_array_layers: self.depth as u32,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D3, // Use D3 for 3D textures
                format: TextureFormat::Rgba8UnormSrgb,
                usage:TextureUsages::all(),
                view_formats: &[TextureFormat::Rgba8UnormSrgb],
            },
            sampler: ImageSampler::default(),
            texture_view_descriptor: None,
        };

        for &value in &self.buffer {
            let pixel_value = (value * 255.0).round() as u8;
            image.data.push(pixel_value);
            image.data.push(pixel_value);
            image.data.push(pixel_value);
            image.data.push(255); // Alpha channel
        }

        image
    }

    fn save(&self, path: String) -> Result<bool, std::io::Error> //returns true if sucessfuly saved and false if not
    {
        if !Path::new(&path).extension().map_or(false, |ext| ext == "noise3d") {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file extension"));
        }

        //format of .noise3d file // 8 Bytes "width", 8 Bytes "height", 8 Bytes "depth", each proceeding 4 Byte chunk is a f32 starting from the 0 position in the "buffer"
        let mut file = File::create(path)?;
        
        // Write dimensions to the file
        file.write_all(&(self.width as u64).to_ne_bytes())?;
        file.write_all(&(self.height as u64).to_ne_bytes())?;
        file.write_all(&(self.depth as u64).to_ne_bytes())?;

        // Write buffer data to the file
        for &value in &self.buffer {
            file.write_all(&value.to_ne_bytes())?;
        }

        Ok(true)
    }

    fn load(path: String) -> Result<NoiseMap3D, std::io::Error> {

        if !Path::new(&path).extension().map_or(false, |ext| ext == "noise3d") {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file extension"));
        }

        let mut file = File::open(path)?;

        // Read dimensions from the file
        let mut width_bytes = [0u8; 8];
        let mut height_bytes = [0u8; 8];
        let mut depth_bytes = [0u8; 8];

        file.read_exact(&mut width_bytes)?;
        file.read_exact(&mut height_bytes)?;
        file.read_exact(&mut depth_bytes)?;

        let width = u64::from_ne_bytes(width_bytes) as usize;
        let height = u64::from_ne_bytes(height_bytes) as usize;
        let depth = u64::from_ne_bytes(depth_bytes) as usize;

        // Read buffer data from the file
        let mut buffer = vec![0.0; width * height * depth];
        for value in &mut buffer {
            let mut value_bytes = [0u8; 4];
            file.read_exact(&mut value_bytes)?;
            *value = f32::from_ne_bytes(value_bytes);
        }

        Ok(NoiseMap3D { width, height, depth, buffer })
    }
    
}


