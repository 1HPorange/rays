pub struct RenderParameters<T> {
    pub quality: QualityParameters<T>,
    pub dof: DoFParameters<T>,
    pub sample_limits: SampleLimits,
    pub ao: AoParameters<T>
}

impl<T> RenderParameters<T> where T: num_traits::Float {

    // TODO: Input validation. Actually, input validation is missing everywhere :'( And think about what struct field can be pub safely

    pub fn new(quality: QualityParameters<T>, dof: DoFParameters<T>, sample_limits: SampleLimits, ao: AoParameters<T>) -> RenderParameters<T> {

        RenderParameters { 
            quality,
            dof,
            sample_limits,
            ao
        }

    }

    pub fn default() -> RenderParameters<T> {
        
        RenderParameters::new(
            QualityParameters {
                min_intensity: T::from(0.03).unwrap(),
                max_bounces: std::i32::MAX,
                float_correction_bias: T::from(0.01).unwrap()
            },
            DoFParameters {
                max_angle: T::from(0.1).unwrap(),
                max_samples: 20
            },
            SampleLimits {
                max_reflection_samples: 5,
                max_refraction_samples: 1
            },
            AoParameters {
                strength: T::from(0.8).unwrap(),
                distance: T::from(2.0).unwrap(),
                samples: 3
            }
        )

    }

}

// Support structs

pub struct QualityParameters<T> {

    pub min_intensity: T,
    pub max_bounces: i32,

    /// Floating point errors can cause visual artifacts in reflections and refraction.
    /// This bias introduces slight inaccuracies with these phenomena, but removes the
    /// artifacts. Basically: Keep lowering this until you see artifacts
    pub float_correction_bias: T

}

// TODO: replace many i32 with u32

pub struct SampleLimits {

    pub max_reflection_samples: i32,
    pub max_refraction_samples: i32
}

pub struct DoFParameters<T> {

    pub max_angle: T,
    pub max_samples: u32

}

pub struct AoParameters<T> {

    pub strength: T,
    pub distance: T,
    pub samples: i32

}