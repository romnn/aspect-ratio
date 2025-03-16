#![warn(missing_docs)]

pub use safe_arithmetic as arithmetic;
pub use safe_arithmetic::Error;

use safe_arithmetic::Cast;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NonUniformScalingFactor {
    pub x: f64,
    pub y: f64,
}

impl Default for NonUniformScalingFactor {
    fn default() -> Self {
        Self { x: 1.0, y: 1.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, strum::EnumIter)]
pub enum ScalingFactor {
    NonUniform(NonUniformScalingFactor),
    Uniform(f64),
}

impl ScalingFactor {
    #[must_use]
    pub fn is_uniform(&self) -> bool {
        matches!(self, Self::Uniform(_))
    }

    #[must_use]
    pub fn as_uniform(&self) -> Option<f64> {
        match self {
            Self::NonUniform { .. } => None,
            Self::Uniform(x) => Some(*x),
        }
    }

    #[must_use]
    pub fn as_non_uniform(&self) -> Option<f64> {
        match self {
            Self::NonUniform { .. } => None,
            Self::Uniform(x) => Some(*x),
        }
    }

    #[must_use]
    pub fn x(&self) -> f64 {
        match self {
            Self::NonUniform(NonUniformScalingFactor { x, .. }) | Self::Uniform(x) => *x,
        }
    }

    #[must_use]
    pub fn y(&self) -> f64 {
        match self {
            Self::NonUniform(NonUniformScalingFactor { y, .. }) | Self::Uniform(y) => *y,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, strum::EnumIter)]
pub enum ScalingMode {
    /// Scale to an exact size.
    ///
    /// If both dimensions are given, aspect ratio is ignored.
    /// If at most a single dimension is given, aspect ratio is kept.
    ///
    /// # Example
    /// ```
    /// use aspect_ratio::{Size, Bounds};
    /// assert_eq!(
    ///     Size::new(200, 200).scale(Bounds::exact().w(300).h(600)).unwrap(),
    ///     Size::new(300, 600)
    /// );
    ///
    /// assert_eq!(
    ///     Size::new(200, 100).scale(Bounds::exact().h(150)).unwrap(),
    ///     Size::new(200, 150)
    /// );
    /// ```
    Exact,

    /// Fit to wxh while keeping aspect ratio, scaling up _or_ down as required.
    ///
    /// If at most one dimension is given, the larger image dimension is scaled to
    /// fit into ``min(w, h)``.
    ///
    /// # Example
    /// ```
    /// use aspect_ratio::{Size, Bounds};
    /// assert_eq!(
    ///     Size::new(200, 200).scale(Bounds::fit().w(400).h(100)).unwrap(),
    ///     Size::new(100, 100)
    /// );
    /// ```
    Fit,

    /// Fit to cover `(w, h)` while keeping aspect ratio.
    ///
    /// If at most one dimension is given, the smallest dimension is scaled up to
    /// cover ``min(w, h)``.
    ///
    /// # Example
    /// ```
    /// use aspect_ratio::{Size, Bounds};
    /// assert_eq!(
    ///     Size::new(200, 200).scale(Bounds::cover().w(400).h(100)).unwrap(),
    ///     Size::new(400, 400)
    /// );
    /// ```
    Cover,

    /// Scale to be contained while keeping aspect ratio, **only** scaling down.
    ///
    ///
    /// # Example
    /// ```
    /// use aspect_ratio::{Size, Bounds};
    /// assert_eq!(
    ///     Size::new(200, 200).scale(Bounds::contain().w(400).h(400)).unwrap(),
    ///     Size::new(200, 200)
    /// );
    /// ```
    Contain,
}

impl Default for ScalingMode {
    fn default() -> Self {
        Self::Contain
    }
}

impl ScalingMode {
    #[must_use]
    pub fn iter() -> <Self as strum::IntoEnumIterator>::Iterator {
        <Self as strum::IntoEnumIterator>::iter()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bounds {
    /// width of the image
    pub width: Option<u32>,
    /// height of the image
    pub height: Option<u32>,
    /// mode of scaling
    pub mode: Option<ScalingMode>,
}

impl Bounds {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn fit() -> Self {
        Self::default().mode(ScalingMode::Fit)
    }

    #[must_use]
    pub fn cover() -> Self {
        Self::default().mode(ScalingMode::Cover)
    }

    #[must_use]
    pub fn contain() -> Self {
        Self::default().mode(ScalingMode::Contain)
    }

    #[must_use]
    pub fn exact() -> Self {
        Self::default().mode(ScalingMode::Exact)
    }

    #[must_use]
    pub fn mode(mut self, mode: impl Into<Option<ScalingMode>>) -> Self {
        self.mode = mode.into();
        self
    }

    #[must_use]
    pub fn w(mut self, width: impl Into<Option<u32>>) -> Self {
        self.width = width.into();
        self
    }

    #[must_use]
    pub fn h(mut self, height: impl Into<Option<u32>>) -> Self {
        self.height = height.into();
        self
    }

    #[must_use]
    pub fn max_width(self, width: impl Into<Option<u32>>) -> Self {
        self.w(width)
    }

    #[must_use]
    pub fn max_height(self, height: impl Into<Option<u32>>) -> Self {
        self.h(height)
    }

    #[must_use]
    pub fn max_dim(self, dim: impl Into<Option<u32>>) -> Self {
        self.max_dimension(dim)
    }

    #[must_use]
    pub fn max_dimension(mut self, dim: impl Into<Option<u32>>) -> Self {
        let dim = dim.into();
        self = self.w(dim);
        self = self.h(dim);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    /// width
    pub width: u32,
    /// height
    pub height: u32,
}

impl Size {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    #[must_use]
    pub fn w(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    #[must_use]
    pub fn h(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    #[must_use]
    pub fn width(self, width: u32) -> Self {
        self.w(width)
    }

    #[must_use]
    pub fn height(self, height: u32) -> Self {
        self.h(height)
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl Size {
    /// Scale size to bounds.
    ///
    /// # Errors
    /// If an arithmetic error (e.g. division by zero) is encountered.
    #[inline]
    pub fn scale(self, bounds: Bounds) -> Result<Self, Error> {
        let mode = bounds.mode.unwrap_or_default();
        match bounds {
            // unbounded
            Bounds {
                width: None,
                height: None,
                ..
            } => Ok(self),
            // single dimension is bounded
            Bounds {
                width: None,
                height: Some(height),
                ..
            } => self.scale_to(
                Size {
                    width: self.width,
                    height,
                },
                mode,
            ),
            Bounds {
                width: Some(width),
                height: None,
                ..
            } => self.scale_to(
                Size {
                    width,
                    height: self.height,
                },
                mode,
            ),
            // all dimensions bounded
            Bounds {
                width: Some(width),
                height: Some(height),
                ..
            } => self.scale_to(Size { width, height }, mode),
        }
    }

    /// Maximum dimension.
    ///
    /// The maximum dimension is computed as the maximum of the width and height.
    #[inline]
    #[must_use]
    pub fn max_dim(&self) -> u32 {
        self.width.max(self.height)
    }

    /// Minimum dimension
    ///
    /// The minimum dimension is computed as the minimum of the width and height.
    #[inline]
    #[must_use]
    pub fn min_dim(&self) -> u32 {
        self.width.min(self.height)
    }

    /// Compute aspect-ratio of the size.
    ///
    /// # Errors
    /// If an arithmetic error (e.g. division by zero) is encountered.
    #[inline]
    pub fn aspect_ratio(&self) -> Result<f64, Error> {
        let width = f64::from(self.width);
        let height = f64::from(self.height);
        let ratio = safe_arithmetic::ops::CheckedDiv::checked_div(width, height)?;
        Ok(ratio)
    }

    /// Compute the scaling factor to scale `self` to the given size.
    ///
    /// # Errors
    /// If an arithmetic error (e.g. division by zero) is encountered.
    #[inline]
    pub fn scaling_factor(
        &self,
        size: impl Into<Size>,
        mode: ScalingMode,
    ) -> Result<ScalingFactor, Error> {
        let target = size.into();
        let target_width = f64::from(target.width);
        let width = f64::from(self.width);
        let target_height = f64::from(target.height);
        let height = f64::from(self.height);

        let width_ratio = safe_arithmetic::ops::CheckedDiv::checked_div(target_width, width)?;
        let height_ratio = safe_arithmetic::ops::CheckedDiv::checked_div(target_height, height)?;

        let factor = match mode {
            ScalingMode::Exact => ScalingFactor::NonUniform(NonUniformScalingFactor {
                x: width_ratio,
                y: height_ratio,
            }),
            ScalingMode::Cover => ScalingFactor::Uniform(f64::max(width_ratio, height_ratio)),
            ScalingMode::Fit => ScalingFactor::Uniform(f64::min(width_ratio, height_ratio)),
            ScalingMode::Contain => {
                ScalingFactor::Uniform(f64::min(f64::min(width_ratio, height_ratio), 1.0))
            }
        };
        Ok(factor)
    }

    /// Scale width and height of `self` by a scaling factor.
    ///
    /// # Errors
    /// If an arithmetic error (e.g. division by zero) is encountered.
    #[inline]
    pub fn scale_by<F, R>(self, sx: F, sy: F) -> Result<Self, Error>
    where
        R: safe_arithmetic::RoundingMode,
        F: safe_arithmetic::Cast + safe_arithmetic::Type,
    {
        let sx: f64 = sx.cast()?;
        let sy: f64 = sy.cast()?;
        let width: f64 = self.width.cast()?;
        let height: f64 = self.height.cast()?;
        let width = safe_arithmetic::ops::CheckedMul::checked_mul(width, sx)?;
        let height = safe_arithmetic::ops::CheckedMul::checked_mul(height, sy)?;
        // todo: should we allow the size to go zero here?
        let width: u32 = R::round(width).cast()?;
        let height: u32 = R::round(height).cast()?;
        Ok(Self { width, height })
    }

    /// Scale `self` to the given size.
    ///
    /// # Errors
    /// If an arithmetic error (e.g. division by zero) is encountered.
    #[inline]
    pub fn scale_to(self, size: impl Into<Size>, mode: ScalingMode) -> Result<Self, Error> {
        let target = size.into();
        if mode == ScalingMode::Exact {
            return Ok(target);
        }
        let factor = self.scaling_factor(target, mode)?;
        let scaled = self.scale_by::<_, safe_arithmetic::Ceil>(factor.x(), factor.y())?;
        Ok(scaled)
    }
}

#[cfg(test)]
mod tests {
    use super::{Bounds, ScalingMode, Size};
    use color_eyre::eyre;
    use similar_asserts::assert_eq as sim_assert_eq;

    static INIT: std::sync::Once = std::sync::Once::new();

    /// Initialize test
    ///
    /// This ensures `color_eyre` is setup once.
    pub fn init() {
        INIT.call_once(|| {
            color_eyre::install().ok();
        });
    }

    #[test]
    fn scale_unbounded() -> eyre::Result<()> {
        crate::tests::init();
        for mode in ScalingMode::iter().map(Some).chain([None]) {
            sim_assert_eq!(
                Size::new(200, 200).scale(Bounds::new().mode(mode))?,
                Size::new(200, 200),
            );
        }
        Ok(())
    }

    // #[test]
    // fn scale_bounded_fill() -> eyre::Result<()> {
    //     crate::tests::init();
    //
    //     // fill 200 x 200 to 300 x * -> 300 x 300
    //     sim_assert_eq!(
    //         Size::new(200, 200).scale(Bounds::new().w(300).mode(ScalingMode::Fill))?,
    //         Size::new(300, 300),
    //     );
    //
    //     // fill 400 x 600 to 300 x 300 -> 300 x 300
    //     sim_assert_eq!(
    //         Size::new(400, 600).scale(Bounds::new().w(300).h(300).mode(ScalingMode::Fill))?,
    //         Size::new(300, 450),
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn scale_bounded_fit() -> eyre::Result<()> {
    //     crate::tests::init();
    //
    //     // fit 200 x 200 to 300 x * -> 300 x 300
    //     sim_assert_eq!(
    //         Size::new(200, 200).scale(Bounds::new().w(300).mode(ScalingMode::Fit))?,
    //         Size::new(300, 300),
    //     );
    //
    //     // fit 400 x 600 to 300 x 300 -> 300 x 300
    //     sim_assert_eq!(
    //         Size::new(400, 600).scale(Bounds::new().w(300).h(300).mode(ScalingMode::Fit))?,
    //         Size::new(200, 300),
    //     );
    //     Ok(())
    // }

    #[test]
    fn scale_bounded_contain() -> eyre::Result<()> {
        crate::tests::init();

        // contain 200 x 200 to 300 x 300 -> 200 x 200
        sim_assert_eq!(
            Size::new(200, 200).scale(Bounds::new().w(300).h(300).mode(ScalingMode::Contain))?,
            Size::new(200, 200)
        );

        // contain 200 x 200 to 200 x * -> 200 x 200
        sim_assert_eq!(
            Size::new(200, 200).scale(Bounds::new().w(200).mode(ScalingMode::Contain))?,
            Size::new(200, 200)
        );

        // contain 200 x 200 to 100 x * -> 100 x 100
        sim_assert_eq!(
            Size::new(200, 200).scale(Bounds::new().w(100).mode(ScalingMode::Contain))?,
            Size::new(100, 100)
        );

        // contain 200 x 200 to 100 x * -> 100 x 100
        sim_assert_eq!(
            Size::new(200, 200).scale(Bounds::new().w(100).mode(ScalingMode::Contain))?,
            Size::new(100, 100)
        );

        // contain 200 x 400 to * x 500 -> 200 x 400
        sim_assert_eq!(
            Size::new(200, 400).scale(Bounds::new().h(500).mode(ScalingMode::Contain))?,
            Size::new(200, 400)
        );

        // contain 200 x 400 to * x 200 -> 100 x 200
        sim_assert_eq!(
            Size::new(200, 400).scale(Bounds::new().h(200).mode(ScalingMode::Contain))?,
            Size::new(100, 200)
        );
        Ok(())
    }
}
