use std::f64::consts::PI;

use shared::debug;

pub enum Formulas{
    Circular,
    Randomized
}

#[allow(unused)]
impl Formulas{
    pub fn convert(self, x_bounds: (f64, f64), y_bounds: (f64, f64), len: usize) -> Vec<(f64, f64)>{
        match self{
            Self::Circular => Self::circular(x_bounds, y_bounds, len),
            Self::Randomized => Self::random(x_bounds, y_bounds, len),
        }
    }

    /// Calculates the position x and y of the node based on a radian. Where:
    /// 
    /// n = existing nodes inside Kalopsia's runtime
    /// 
    /// i = the current index of the iteration
    /// 
    /// r = radius of the formed circle
    /// 
    /// cx = x coordinate of which the circle will be centered
    /// 
    /// cy = y coordinate of which the circle will be centered
    /// 
    /// # How the calculus is done
    /// ```plaintext
    /// angle = 2π * i/n
    /// 
    /// x = cx + r * cos(angle)
    /// y = cy + r * sin(angle)
    /// ```
    /// 
    /// # Panics
    /// Panics if the current amount of nodes inside Kalopsia's runtime is 0.
    fn circular(x_bounds: (f64, f64), y_bounds: (f64, f64), len: usize) -> Vec<(f64, f64)>{
        let centered = ((x_bounds.1 - x_bounds.0) + (y_bounds.1 - y_bounds.0)) / 4.0;
        debug!("{}", centered);
        let radius = 10.0;
        let mut pos: Vec<(f64, f64)> = Vec::new();

        for i in 0..len{
            let angle = 2.0 * PI * i as f64 / len as f64;

            pos.push((
                centered + radius * angle.cos(),
                centered + radius * angle.sin()
            ));
        }
        debug!("{:#?}", pos);
        pos
    }

    pub fn random(x_bounds: (f64, f64), y_bounds: (f64, f64), len: usize) -> Vec<(f64, f64)>{
        todo!()
    }
}