
use std::{clone, fs::File, io::{Write, Read}, path::Path, usize};

use libnoise::prelude::*;
use bevy::{prelude::*, math::f32, render::{texture::ImageSampler, render_resource::{TextureDescriptor, TextureDimension, TextureFormat, Extent3d, TextureUsages}}};

const DEPTH_SCALAR: f64 = 0.142;

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
impl NoiseSource2D for CombinedNoiseGenerator2D {
    fn sample(&self, point: [f64; 2]) -> f64 {
        return CombinedNoiseGenerator2D::get_weighted_value(&self, point[0] as i32, point[1] as i32) as f64;
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
    pub perlin_generator: NoiseGenerator2D<Perlin<2>>,
    pub simplex_generator: NoiseGenerator2D<Simplex<2>>,
    pub worley_generator: NoiseGenerator2D<Worley<2>>,
    pub perlin_weight: f32,
    pub simplex_weight: f32,
    pub worley_weight: f32,
}
impl CombinedNoiseGenerator2D
{
    pub fn get_value(&self, x: i32, y: i32) -> f32
    {
        let perlin = self.perlin_generator.get_value_at(x, y);
        let simplex = self.simplex_generator.get_value_at(x, y);
        let worley = self.worley_generator.get_value_at(x, y);
        
        return perlin + simplex + worley;
    }
    pub fn get_weighted_value(&self, x: i32, y: i32) -> f32
    {
        let perlin = self.perlin_generator.get_value_at(x, y) * self.perlin_weight;
        let simplex = self.simplex_generator.get_value_at(x, y) * self.simplex_weight;
        let worley = self.worley_generator.get_value_at(x, y) * self.worley_weight;
        let ratio = 1.0 / (self.perlin_weight + self.simplex_weight + self.worley_weight);
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
            let new_z: f32 = z as f32 * frequency * DEPTH_SCALAR as f32;
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
    pub perlin_generator: NoiseGenerator3D<Perlin<3>>,
    pub simplex_generator: NoiseGenerator3D<Simplex<3>>,
    pub worley_generator: NoiseGenerator3D<Worley<3>>,
    pub perlin_weight: f32,
    pub simplex_weight: f32,
    pub worley_weight: f32,
}
impl CombinedNoiseGenerator3D
{
    pub fn get_value(&self, x: i32, y: i32, z: i32) -> f32
    {
        let perlin = self.perlin_generator.get_value_at(x, y, z );
        let simplex = self.simplex_generator.get_value_at(x, y, z);
        let worley = self.worley_generator.get_value_at(x, y, z);
        
        return perlin + simplex + worley;
    }
    pub fn get_weighted_value(&self, x: i32, y: i32, z: i32) -> f32
    {
        let perlin = self.perlin_generator.get_value_at(x, y, z) * self.perlin_weight;
        let simplex = self.simplex_generator.get_value_at(x, y, z) * self.simplex_weight;
        let worley = self.worley_generator.get_value_at(x, y, z) * self.worley_weight;
        let ratio = 1.0 / (self.perlin_weight + self.simplex_weight + self.worley_weight);
        return perlin * ratio + simplex * ratio + worley * ratio;
    }
}


//-------------------------------------------------------------------Defining Noise Map 2D-------------------------------------------------------------------//
#[derive(Resource)]
pub struct NoiseMap2D
{
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<f32>
} 
impl NoiseMap2D
{   
    pub fn from_values(width: usize, height: usize) -> NoiseMap2D
    {
        let buffer = vec![0.0 as f32; width * height];                 
        return NoiseMap2D{ width, height, buffer};
    }
    pub fn initialize(&mut self)
    {
        self.buffer = vec![0.0; self.width * self.height]        
    }
    pub fn get_value_at(&self, x: u32, y: u32) -> f32
    {
        return self.buffer[((y as usize * self.height) + x as usize)]
    }
    pub fn set_value_at(&mut self, x: u32, y: u32, value: f32)
    {
        self.buffer[((y as usize * self.height) + x as usize)] = value;
    }
    pub fn to_image(&self) -> Image {
        let mut image = Image {
            data: Vec::with_capacity(self.buffer.len()),
            texture_descriptor: TextureDescriptor {
                label: None,
                size: Extent3d{width: self.width as u32, height: self.height as u32, depth_or_array_layers: 1},
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8Unorm,
                usage: TextureUsages::all(),
                view_formats: &[TextureFormat::Rgba8Unorm],                
            },
            sampler: ImageSampler::default(),
            texture_view_descriptor: None,
        };

        for &value in &self.buffer {
            
            let pixel_value = (value * 255.0) as u8;
            //println!("Assigning pixel value: {} with buffer value: {}", pixel_value, value);
            image.data.push(pixel_value);
            image.data.push(pixel_value);
            image.data.push(pixel_value);
            image.data.push(255); // Alpha channel
        }

        image
    }
    
    pub fn save(&self, path: String) -> Result<bool, std::io::Error> //returns true if sucessfuly saved and false if not
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
    
    pub fn load(path: &str) -> Result<NoiseMap2D, std::io::Error> {

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
#[derive(Resource)]
pub struct NoiseMap3D
{
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub buffer: Vec<f32>
}
impl NoiseMap3D
{
    pub fn from_values(width: usize, height: usize, depth: usize) -> NoiseMap3D
    {
        let buffer = vec![0.0 as f32; width * height * depth];                 
        return NoiseMap3D{ width, height, depth, buffer};
    }

    pub fn initialize(&mut self)
    {
        self.buffer = vec![0.0; self.width * self.height * self.depth];        
    }

    pub fn get_value_at(&self, x: u32, y: u32, z: u32) -> f32
    {
        return self.buffer[(z as usize * self.height * self.width) + (y as usize * self.height) + x as usize]
    }

    pub fn set_value_at(&mut self, x: u32, y: u32, z: u32, value: f32)
    {   
        self.buffer[(z as usize * self.height * self.width) + (y as usize * self.height) + x as usize] = value;
    }

    pub fn populate(&mut self, noise_generator: &CombinedNoiseGenerator3D)
    {
        if self.buffer.is_empty()
        {
            self.initialize();
        }
        for z in 0..self.depth
        {
            for y in 0..self.height
            {
                for x in 0..self.width
                {
                   self.buffer[(z * self.height * self.width) + (y * self.height as usize) + x] = noise_generator.get_weighted_value(x as i32, y as i32, z  as i32);
                }
            }
        }

    }
    pub fn populate_from_perlin(&mut self, perlin_generator: &NoiseGenerator3D<Perlin<3>>)
    {
        if self.buffer.is_empty()
        {
            self.initialize();
        }
        for z in 0..self.depth
        {
            for y in 0..self.height
            {
                for x in 0..self.width
                {
                   self.buffer[(z * self.height * self.width) + (y * self.height as usize) + x] = perlin_generator.get_value_at(x as i32, y as i32, z as i32);
                }
            }
        }
    }
    pub fn populate_from_simplex(&mut self, simplex_generator: &NoiseGenerator3D<Simplex<3>>)
    {
        if self.buffer.is_empty()
        {
            self.initialize();
        }
        for z in 0..self.depth
        {
            for y in 0..self.height
            {
                for x in 0..self.width
                {
                   self.buffer[(z * self.height * self.width) + (y * self.height as usize) + x] = simplex_generator.get_value_at(x as i32, y as i32, z as i32);
                }
            }
        }
    }
    pub fn populate_from_worley(&mut self, worley_generator: &NoiseGenerator3D<Worley<3>>)
    {
        if self.buffer.is_empty()
        {
            self.initialize();
        }
        for z in 0..self.depth
        {
            for y in 0..self.height
            {
                for x in 0..self.width
                {
                   self.buffer[(z * self.height * self.width) + (y * self.height as usize) + x] = worley_generator.get_value_at(x as i32, y as i32, z as i32);
                }
            }
        }
    }

    pub fn get_slice_x(&self, x_level: u32) -> NoiseMap2D
    {
        let mut buffer = Vec::with_capacity(self.depth * self.height);
        for z in 0..self.depth
        {
            for y in 0..self.height
            {
                buffer[z * self.depth + y] = self.get_value_at(x_level, y as u32, z as u32);
            }
        }

        return  NoiseMap2D{ width: self.height, height: self.depth, buffer };
    }
    pub fn get_slice_y(&self, y_level: u32) -> NoiseMap2D
    {
        let mut buffer = Vec::with_capacity(self.depth * self.height);
        for z in 0..self.depth
        {
            for x in 0..self.width
            {
                buffer[z * self.depth + x] = self.get_value_at(x as u32, y_level, z as u32);
            }
        }

        return  NoiseMap2D{ width: self.height, height: self.depth, buffer };
    }
    pub fn get_slice_z(&self, z_level: u32) -> NoiseMap2D
    {
        let mut temp_buffer: Vec<f32> = vec![0.0 as f32; self.width * self.height];
        
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                //println!("Assigning pixel x:{}, y:{} . Value: {}",x,y, self.get_value_at(x as u32, y as u32, z_level));
                temp_buffer[y * self.height + x] = self.get_value_at(x as u32, y as u32, z_level)
            }
        }

        return  NoiseMap2D{ width: self.width, height: self.height, buffer: temp_buffer };
    }


    pub fn to_image(&self) -> Image {
        let mut image = Image {
            data: Vec::with_capacity(self.buffer.len()),
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
                format: TextureFormat::Rgba8Unorm,
                usage:TextureUsages::all(),
                view_formats: &[TextureFormat::Rgba8Unorm],
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

    pub fn save(&self, path: String) -> Result<bool, std::io::Error> //returns true if sucessfuly saved and false if not
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

    pub fn load(path: String) -> Result<NoiseMap3D, std::io::Error> {

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


