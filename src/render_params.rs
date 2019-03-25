use crate::util;

pub struct RenderParams<T> {
    pub quality: QualityParameters<T>,
    pub dof: DoFParameters<T>,
    pub max_samples: MaxSamples,
    pub ao: AoParameters<T>
}

impl<T> Default for RenderParams<T> where T: num_traits::Float {

    fn default() -> Self {

        RenderParams {
            quality: QualityParameters {
                min_intensity: T::from(0.03).unwrap(),
                max_bounces: std::u32::MAX,
                bias: T::from(0.01).unwrap()
            },
            dof: DoFParameters {
                max_angle: T::from(0.1).unwrap(),
                samples: 10
            },
            max_samples: MaxSamples {
                reflection: 6,
                refraction: 1
            },
            ao: AoParameters {
                strength: T::from(0.8).unwrap(),
                distance: T::from(2.0).unwrap(),
                samples: 3
            }
        }

    }

}

impl<T> RenderParams<T> where T: num_traits::Float {

    pub fn validate(&self) -> bool {

        let mut success = true;

        // Quality settings

        if !util::is_in_range(self.quality.min_intensity, T::zero(), T::one()) {
            println!("Error: Minimum intensity needs to be within 0-1 range");
            success = false;
        }

        if self.quality.max_bounces == 0 {
            println!("Warning: Reflections won't work with 0 max_bounces");
        }

        if self.quality.max_bounces < 2 {
            println!("Warning: Refraction won't work properly with less than 2 max_bounces");
        }

        if !util::is_in_range(self.quality.bias, T::zero(), T::infinity()) {
            println!("Error: Float correction bias must be 0 or positive");
            success = false;
        }

        // DoF

        if !util::is_in_range(self.dof.max_angle, T::zero(), T::from(360.0).unwrap()) {
            println!("Error: dof.max_angle needs to be between 0 and 360 degrees");
            success = false;
        }

        if !self.dof.max_angle.is_zero() && self.dof.samples == 0 {
            println!("Warning: Image will render black because of zero DoF samples, but non-zero DoF max angle.")
        }

        // Sample Limits

        if self.max_samples.reflection == 0 {
            println!("Warning: Reflections will not work when max_reflection_samples is 0");
        }

        if self.max_samples.refraction == 0 {
            println!("Warning: Refraction won't work when max_refraction_samples is 0");
        }

        // Ao

        if !util::is_in_range(self.ao.strength, T::zero(), T::one()) {
            println!("Error: AO strength must be in range 0-1");
            success = false;
        }

        if !util::is_in_range(self.ao.distance, T::zero(), T::infinity()) {
            println!("Error: AO distance must be 0 or positive");
            success = false;
        }

        if self.ao.strength > T::zero() {

            if self.ao.distance.is_zero() {
                println!("Warning: AO will not work if distance is 0");
            }

            if self.ao.samples == 0 {
                println!("Warning: AO will not work if samples are 0");
            }
        }

        success
    }

}

// Support structs

pub struct QualityParameters<T> {

    /// Range: 0-1
    /// A ray is not allowed to spawn more rays if its total intensity falls below
    /// this limit. Setting this value often leads to prettier results than setting
    /// max_bounces directly
    pub min_intensity: T,

    /// How often the raytracing function is allowed to recurse. If set to 0, no
    /// reflective and refractive effects will be visible at all.
    /// You can safely set this to std::u32::MAX if you set min_intensity instead
    pub max_bounces: u32,

    /// Floating point errors can cause visual artifacts in reflections and refraction.
    /// This bias introduces slight inaccuracies with these phenomena, but removes the
    /// artifacts. Basically: Keep lowering this until you see artifacts
    pub bias: T
}

pub struct MaxSamples {
 
    /// Maximum number of rays that might be sent out when a reflective surface is hit
    pub reflection: u32,

    /// Maximum number of rays that might be sent out when a refractive surface is hit
    pub refraction: u32
}

pub struct DoFParameters<T> {

    /// Sensible Range: Low single digit degrees
    /// Unit: Degrees
    /// Maximum angle deviation to the direction that an initial camera ray can get.
    /// If set to zero, dof.samples is ignored and a single ray is sent out.
    pub max_angle: T,

    /// Number of samples that each pixel in the final image consists of. This setting
    /// is ignored (and treated as 1) when max_angle is set to 0
    pub samples: u32

}

pub struct AoParameters<T> {

    /// Range: 0-1
    /// How black the AO shadows are
    pub strength: T,

    /// Range: Positive World units
    /// Fallof range of AO shadows
    pub distance: T,

    /// How many sample rays are sent out to estimate AO
    pub samples: u32

}