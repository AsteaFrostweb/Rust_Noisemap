
use std::clone;

use libnoise::prelude::*;
use bevy::{prelude::*, math::f32};




#[derive(Clone)]
pub struct NoiseValues
{
    pub octaves: usize,
    pub scale: f32,
    pub persistence: f32,
    pub lacunarity: f32
}



#[derive(Resource, Clone)]
pub struct NoiseGenerator
{
    pub perlin_source: Perlin<2>,
    pub simplex_source: Simplex<2>,
    pub worley_source: Worley<2>,
    pub perlin_noise_values: NoiseValues,
    pub simplex_noise_values: NoiseValues,
    pub worley_noise_values: NoiseValues
}
impl NoiseGenerator {    
  
    
    pub fn get_combined_value_at_weighted(&self, x: i32, y: i32, perlin_weight: f32, simplex_weight: f32, worley_weight: f32) -> f32
    {
        let perlin = self.get_perlin_value_at(x, y) * perlin_weight;
        let simplex = self.get_simplex_value_at(x, y) * simplex_weight;
        let worley = self.get_worley_value_at(x, y) * worley_weight;
        let ratio = 1.0 / (perlin_weight + simplex_weight + worley_weight);
        return perlin * ratio + simplex * ratio + worley * ratio;
    }


    pub fn get_perlin_value_at(&self, x: i32, y: i32) -> f32 {

        let mut sum = 0.0;
        let mut amplitude: f32 = 1.0;
        let mut frequency = self.perlin_noise_values.scale;
        l
        for _ in 0..self.perlin_noise_values.octaves {
            let new_x: f32 = x as f32 * frequency;
            let new_y: f32 = y as f32 * frequency;
            let sample: f64 = self.perlin_source.sample([new_x as f64, new_y as f64]);
            sum += sample as f32 * amplitude;
            
            amplitude *= self.perlin_noise_values.persistence;
            frequency *= self.perlin_noise_values.lacunarity;
        }
       
        return sum;
    }


    pub fn get_simplex_value_at(&self, x: i32, y: i32) -> f32 {

        let mut sum: f32 = 0.0;
        let mut amplitude: f32 = 1.0;
        let mut frequency: f32 = self.simplex_noise_values.scale;
        let mut max_amplitude = 1.0;                                        

        for _ in 0..self.simplex_noise_values.octaves {
            let new_x: f32 = x as f32 * frequency;
            let new_y: f32 = y as f32 * frequency;
            let sample: f32 = self.simplex_source.sample([new_x as f64, new_y as f64]) as f32;
            sum += sample * amplitude;

            max_amplitude += amplitude;
            amplitude *= self.simplex_noise_values.persistence;
            frequency *= self.simplex_noise_values.lacunarity;
        }
       
        return sum as f32;
    }


    pub fn get_worley_value_at(&self, x: i32, y: i32) -> f32 {

        let mut sum: f32 = 0.0;
        let mut amplitude: f32 = 1.0;
        let mut frequency: f32  = self.worley_noise_values.scale;
                                                     

        for _ in 0..self.worley_noise_values.octaves {
            let new_x: f32 = x as f32 * frequency;
            let new_y: f32 = y as f32 * frequency;
            let sample: f32 = self.worley_source.sample([new_x as f64, new_y as f64]) as f32;
            sum += sample * amplitude;

            
            amplitude *= self.worley_noise_values.persistence;
            frequency *= self.worley_noise_values.lacunarity;
        }
       
        return sum as f32;
    }

}


pub struct NoiseMap2D
{
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<f32>
} 
impl NoiseMap3D
{   
    fn initialize(&self)
    {
        self.buffer = Vec::with_capacity(self.width * self.height)
        
    }
    fn get_value_at(x: u32, y: u32)
    {

    }
    fn set_value_at(x: u32, y: u32)
    {

    }
}

pub struct NoiseMap3D
{
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub buffer: Vec<f32>
}
impl NoiseMap3D
{

}


