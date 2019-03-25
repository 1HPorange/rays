use crate::util;

pub struct RenderParams {
    pub quality: QualityParameters,
    pub dof: DoFParameters,
    pub max_samples: MaxSamples,
    pub ao: AoParameters
}

impl Default for RenderParams {

    fn default() -> Self {

        RenderParams {
            quality: QualityParameters {
                min_intensity: 0.03,
                max_bounces: std::u32::MAX,
                bias: 0.01
            },
            dof: DoFParameters {
                max_angle: 0.1,
                samples: 10
            },
            max_samples: MaxSamples {
                reflection: 6,
                refraction: 1
            },
            ao: AoParameters {
                strength: 0.8,
                distance: 2.0,
                samples: 3
            }
        }

    }

}

impl RenderParams {

    pub fn validate(&self) -> bool {

        let mut success = true;

        // Quality settings

        if !util::is_in_range(self.quality.min_intensity, 0.0, 1.0) {
            println!("Error: Minimum intensity needs to be within 0-1 range");
            success = false;
        }

        if self.quality.max_bounces == 0 {
            println!("Warning: Reflections won't work with 0 max_bounces");
        }

        if self.quality.max_bounces < 2 {
            println!("Warning: Refraction won't work properly with less than 2 max_bounces");
        }

        if !util::is_in_range(self.quality.bias, 0.0, std::f64::INFINITY) {
            println!("Error: Float correction bias must be 0 or positive");
            success = false;
        }

        // DoF

        if !util::is_in_range(self.dof.max_angle, 0.0, 360.0) {
            println!("Error: dof.max_angle needs to be between 0 and 360 degrees");
            success = false;
        }

        if self.dof.max_angle != 0.0 && self.dof.samples == 0 {
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

        if !util::is_in_range(self.ao.strength, 0.0, 1.0) {
            println!("Error: AO strength must be in range 0-1");
            success = false;
        }

        if !util::is_in_range(self.ao.distance, 0.0, std::f64::INFINITY) {
            println!("Error: AO distance must be 0 or positive");
            success = false;
        }

        if self.ao.strength > 0.0 {

            if self.ao.distance == 0.0 {
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

pub struct QualityParameters {

    /// Range: 0-1
    /// A ray is not allowed to spawn more rays if its total intensity falls below
    /// this limit. Setting this value often leads to prettier results than setting
    /// max_bounces directly
    pub min_intensity: f64,

    /// How often the raytracing function is allowed to recurse. If set to 0, no
    /// reflective and refractive effects will be visible at all.
    /// You can safely set this to std::u32::MAX if you set min_intensity instead
    pub max_bounces: u32,

    /// Floating point errors can cause visual artifacts in reflections and refraction.
    /// This bias introduces slight inaccuracies with these phenomena, but removes the
    /// artifacts. Basically: Keep lowering this until you see artifacts
    pub bias: f64
}

pub struct MaxSamples {
 
    /// Maximum number of rays that might be sent out when a reflective surface is hit
    pub reflection: u32,

    /// Maximum number of rays that might be sent out when a refractive surface is hit
    pub refraction: u32
}

pub struct DoFParameters {

    /// Sensible Range: Low single digit degrees
    /// Unit: Degrees
    /// Maximum angle deviation to the direction that an initial camera ray can get.
    /// If set to zero, dof.samples is ignored and a single ray is sent out.
    pub max_angle: f64,

    /// Number of samples that each pixel in the final image consists of. This setting
    /// is ignored (and treated as 1) when max_angle is set to 0
    pub samples: u32

}

pub struct AoParameters {

    /// Range: 0-1
    /// How black the AO shadows are
    pub strength: f64,

    /// Range: Positive World units
    /// Fallof range of AO shadows
    pub distance: f64,

    /// How many sample rays are sent out to estimate AO
    pub samples: u32

}