use crate::util;
use crate::color::RGBColor;
use serde::Deserialize;

// This macro generates a deserializable version of the structs where any member is
// wrapped inside of an Option. This allows us to deserialize a kind of 'override-struct'
// which can then be used to override only specific values inside of a render-params struct

macro_rules! generate_optional_variant {
    (
    $(
    $(#[$outer:meta])*    
    pub struct $name:ident 
    {
        $(
        $(#[$inner:meta])*    
        pub $field:ident : $t:ty
        ),* 
    })*
    ) => {
        $(
        #[derive(Copy, Clone, Debug, Deserialize)]
        #[serde(default)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            $(
            $(#[$inner])*    
            pub $field : $t,
            )* 
        }
        
        impl $name {
            pub fn override_with(&mut self, or: override_structs::$name) {
                $(
                    if let Some($field) = or.$field {
                        self.$field = $field;
                    }
                )*
            }
        }
        )*

        pub mod override_structs {
            use serde::Deserialize;

            $(
            #[derive(Default)]
            #[derive(Copy, Clone, Debug, Deserialize)]
            #[serde(default)]
            #[serde(deny_unknown_fields)] 
            pub struct $name {
                $(  
                pub $field : Option<$t>,
                )* 
            })*
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct RenderParams {
    pub quality: QualityParameters,
    pub dof: DoFParameters,
    pub max_samples: MaxSamples,
    pub ao: AoParameters,

    // This is the color returned when a ray doesn't hit anything
    // If you want a more complex skybox, add it manually as an object
    pub sky_color: RGBColor
}

#[derive(Copy, Clone, Default, Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct RenderParamsOverride {
    pub quality: override_structs::QualityParameters,
    pub dof: override_structs::DoFParameters,
    pub max_samples: override_structs::MaxSamples,
    pub ao: override_structs::AoParameters,

    #[serde(rename = "sky-color")]
    pub sky_color: Option<RGBColor>
}

impl RenderParams {
    pub fn override_with(mut self, or: &RenderParamsOverride) -> RenderParams {
        self.quality.override_with(or.quality);
        self.dof.override_with(or.dof);
        self.max_samples.override_with(or.max_samples);
        self.ao.override_with(or.ao);
        
        if let Some(sky_color) = or.sky_color {
            self.sky_color = sky_color;
        }

        self
    }
}

generate_optional_variant!(
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

);

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

        success = success && self.sky_color.validate();

        success
    }

    pub fn preset_sketch() -> RenderParams {
        
        let mut rp = RenderParams::default();

        rp.quality.min_intensity = 1.0;
        rp.quality.max_bounces = 0;
        rp.max_samples.reflection = 0;
        rp.max_samples.refraction = 0;
        rp.dof.max_angle = 0.0;
        rp.dof.samples = 1;
        rp.ao.strength = 0.0;
        rp.ao.samples = 0;

        rp
    }

    pub fn preset_low() -> RenderParams {
        
        let mut rp = RenderParams::default();

        rp.quality.min_intensity = 0.03;
        rp.quality.max_bounces = 2;
        rp.ao.strength = 0.0;
        rp.ao.samples = 1;

        rp
    }

    pub fn preset_medium() -> RenderParams {
        RenderParams::default()
    }

    pub fn preset_high() -> RenderParams {
        
        let mut rp = RenderParams::default();

        rp.quality.max_bounces = 6;
        rp.dof.samples = 40;
        rp.ao.samples = 4;

        rp
    }

    pub fn preset_ultra() -> RenderParams {

        let mut rp = RenderParams::default();

        rp.quality.min_intensity = 0.01;
        rp.quality.max_bounces = std::u32::MAX;
        rp.dof.samples = 70;
        rp.ao.samples = 6;

        rp
    }
}

// Support structs

impl Default for QualityParameters {
    fn default() -> Self {
        QualityParameters {
            min_intensity: 0.03,
            max_bounces: 4,
            bias: 0.0001
        }
    }
}

impl Default for MaxSamples {
    fn default() -> Self {
        MaxSamples {
            reflection: 3,
            refraction: 1
        }
    }
}

impl Default for DoFParameters {
    fn default() -> Self {
        DoFParameters {
            max_angle: 0.1,
            samples: 20
        }
    }
}

impl Default for AoParameters {
    fn default() -> Self {
        AoParameters {
            strength: 0.8,
            distance: 2.0,
            samples: 2
        }
    }
}