use std::f32::consts::PI;

use egui::{Color32, Painter, Pos2, Shape, Stroke, Vec2, epaint::CubicBezierShape};

const PI_OVER_2: f32 = PI / 2.0;

struct AngleIter {
    start: Option<f32>,
    end: f32,
}

impl AngleIter {
    const fn new(start_angle: f32, end_angle: f32) -> Self {
        Self {
            start: Some(start_angle),
            end: end_angle,
        }
    }
}

impl Iterator for AngleIter {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        self.start.map(|start| {
            let diff = self.end - start;
            if diff.abs() < PI_OVER_2 {
                self.start = None;
                (start, self.end)
            } else {
                let new_start = start + (PI_OVER_2 * diff.signum());
                self.start = Some(new_start);
                (start, new_start)
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            0,
            self.start
                .map(|start| ((self.end - start).abs() / PI_OVER_2).ceil() as usize),
        )
    }
}

pub fn generate_arc(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    stroke: impl Into<Stroke>,
) {
    let stroke = stroke.into();
    painter.extend(
        AngleIter::new(start_angle, end_angle).map(|(start_angle, end_angle)| {
            // Center of the circle
            let xc = center.x;
            let yc = center.y;

            // First control point
            let p1 = center + radius * Vec2::new(start_angle.cos(), -start_angle.sin());

            // Last control point
            let p4 = center + radius * Vec2::new(end_angle.cos(), -end_angle.sin());

            let a = p1 - center;
            let b = p4 - center;
            let q1 = a.length_sq();
            let q2 = q1 + a.dot(b);
            let k2 = (4.0 / 3.0) * ((2.0 * q1 * q2).sqrt() - q2) / (a.x * b.y - a.y * b.x);

            let p2 = Pos2::new(xc + a.x - k2 * a.y, yc + a.y + k2 * a.x);
            let p3 = Pos2::new(xc + b.x + k2 * b.y, yc + b.y - k2 * b.x);

            Shape::CubicBezier(CubicBezierShape::from_points_stroke(
                [p1, p2, p3, p4],
                false,
                Color32::TRANSPARENT,
                stroke,
            ))
        }),
    );
}
