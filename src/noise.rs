
use std::clone;

use libnoise::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;



use super::basic::*;

#[derive(Clone)]
pub struct NoiseValues
{
    pub octaves: usize,
    pub scale: f64,
    pub persistence: f64,
    pub lacunarity: f64
}

#[derive(Resource, Clone)]
pub struct TerrainNoiseGenerator
{
    pub biome_generator: NoiseGenerator,
    pub height_generator: NoiseGenerator
}
impl TerrainNoiseGenerator
{
    pub fn update(&mut self, custom_biome_generator: NoiseGenerator, custom_height_generator: NoiseGenerator)
    {                
        self.biome_generator = custom_biome_generator;
        self.height_generator = custom_height_generator;
    }    
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
  
    pub fn get_combined_value_at(&self, x: i32, y: i32) -> f64
    {
        let perlin = self.get_perlin_value_at(x, y);
        let simplex = self.get_simplex_value_at(x, y);
        let worley = self.get_worley_value_at(x, y);
        return (perlin + simplex + worley) / 3.0;
    }
    pub fn get_combined_value_at_weighted(&self, x: i32, y: i32, perlin_weight: f64, simplex_weight: f64, worley_weight: f64) -> f64
    {
        let perlin = self.get_perlin_value_at(x, y) * perlin_weight;
        let simplex = self.get_simplex_value_at(x, y) * simplex_weight;
        let worley = self.get_worley_value_at(x, y) * worley_weight;
        let ratio = 1.0 / (perlin_weight + simplex_weight + worley_weight);
        return perlin * ratio + simplex * ratio + worley * ratio;
    }

    pub fn get_perlin_value_at(&self, x: i32, y: i32) -> f64 {

        let mut sum = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = self.perlin_noise_values.scale;
        let mut max_amplitude = 1.0;      

     
        


        for _ in 0..self.perlin_noise_values.octaves {
            let new_x: f64 = x as f64 * frequency;
            let new_y: f64 = y as f64 * frequency;
            let sample: f64 = self.perlin_source.sample([new_x, new_y]);
            sum += sample * amplitude;

            max_amplitude += amplitude;
            amplitude *= self.perlin_noise_values.persistence;
            frequency *= self.perlin_noise_values.lacunarity;
        }

       
        return sum;
    }

    pub fn get_simplex_value_at(&self, x: i32, y: i32) -> f64 {

        let mut sum = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = self.simplex_noise_values.scale;
        let mut max_amplitude = 1.0;      

        
        


        for _ in 0..self.simplex_noise_values.octaves {
            let new_x: f64 = x as f64 * frequency;
            let new_y: f64 = y as f64 * frequency;
            let sample: f64 = self.simplex_source.sample([new_x, new_y]);
            sum += sample * amplitude;

            max_amplitude += amplitude;
            amplitude *= self.simplex_noise_values.persistence;
            frequency *= self.simplex_noise_values.lacunarity;
        }

       
        return sum;
    }

    pub fn get_worley_value_at(&self, x: i32, y: i32) -> f64 {

        let mut sum = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = self.worley_noise_values.scale;
        let mut max_amplitude = 1.0;      

        
        


        for _ in 0..self.worley_noise_values.octaves {
            let new_x: f64 = x as f64 * frequency;
            let new_y: f64 = y as f64 * frequency;
            let sample: f64 = self.worley_source.sample([new_x, new_y]);
            sum += sample * amplitude;

            max_amplitude += amplitude;
            amplitude *= self.worley_noise_values.persistence;
            frequency *= self.worley_noise_values.lacunarity;
        }

       
        return -sum;
    }

}





