pub struct RenderParameters<T> {
    pub quality: QualityParameters<T>,
    pub dof: DoFParameters<T>,
    pub sample_limits: SampleLimits,
    pub ao: AoParameters<T>
}

impl<T> Default for RenderParameters<T> where T: num_traits::Float {

    fn default() -> Self {

        RenderParameters {
            quality: QualityParameters {
                min_intensity: T::from(0.03).unwrap(),
                max_bounces: std::u32::MAX,
                float_correction_bias: T::from(0.01).unwrap()
            },
            dof: DoFParameters {
                max_angle: T::from(0.1).unwrap(),
                samples: 10
            },
            sample_limits: SampleLimits {
                max_reflection_samples: 6,
                max_refraction_samples: 1
            },
            ao: AoParameters {
                strength: T::from(0.8).unwrap(),
                distance: T::from(2.0).unwrap(),
                samples: 3
            }
        }

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
    pub float_correction_bias: T
}

pub struct SampleLimits {
 
    /// Maximum number of rays that might be sent out when a reflective surface is hit
    pub max_reflection_samples: u32,

    /// Maximum number of rays that might be sent out when a refractive surface is hit
    pub max_refraction_samples: u32
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