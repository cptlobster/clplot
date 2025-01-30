use crate::renderer::plot::Plot;
use crate::renderer::shapes::ScaledViewBox;

pub enum Axis {
    Manual {
        name: String,
        min: f32,
        max: f32,
        markers: f32
    },
    // Scaled {
    //     name: String,
    //     marker_scale: i8
    // },
}

/// Base struct for a chart.
pub struct BaseChart {
    plot: Plot,
    title: String,
    x: Axis,
    y: Axis,
}

impl BaseChart {
    pub fn new(plot: Plot) -> BaseChart {
        BaseChart {
            plot,
            title: "".to_string(),
            x: Axis::Manual{ name: "".to_string(), min: 0.0, max: 1.0, markers: 0.2 },
            y: Axis::Manual{ name: "".to_string(), min: 0.0, max: 1.0, markers: 0.2 }
        }
    }
}