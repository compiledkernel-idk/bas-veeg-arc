use macroquad::prelude::*;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if a != b {
        (value - a) / (b - a)
    } else {
        0.0
    }
}

pub fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = inverse_lerp(from_min, from_max, value);
    lerp(to_min, to_max, t)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn ease_in_quad(t: f32) -> f32 {
    t * t
}

pub fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

pub fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

pub fn ease_in_elastic(t: f32) -> f32 {
    const C4: f32 = 2.0 * std::f32::consts::PI / 3.0;

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -(2.0_f32.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * C4).sin()
    }
}

pub fn ease_out_elastic(t: f32) -> f32 {
    const C4: f32 = 2.0 * std::f32::consts::PI / 3.0;

    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
    }
}

pub fn ease_out_bounce(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

pub fn angle_between(from: Vec2, to: Vec2) -> f32 {
    (to - from).y.atan2((to - from).x)
}

pub fn rotate_point(point: Vec2, center: Vec2, angle: f32) -> Vec2 {
    let s = angle.sin();
    let c = angle.cos();
    let translated = point - center;
    let rotated = Vec2::new(
        translated.x * c - translated.y * s,
        translated.x * s + translated.y * c,
    );
    rotated + center
}

pub fn circle_intersects_rect(
    circle_pos: Vec2,
    radius: f32,
    rect_pos: Vec2,
    rect_size: Vec2,
) -> bool {
    let closest_x = circle_pos.x.clamp(rect_pos.x, rect_pos.x + rect_size.x);
    let closest_y = circle_pos.y.clamp(rect_pos.y, rect_pos.y + rect_size.y);
    let distance = (circle_pos - Vec2::new(closest_x, closest_y)).length();
    distance < radius
}

pub fn line_intersects_circle(start: Vec2, end: Vec2, circle_pos: Vec2, radius: f32) -> bool {
    let d = end - start;
    let f = start - circle_pos;

    let a = d.dot(d);
    let b = 2.0 * f.dot(d);
    let c = f.dot(f) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

pub fn wrap_angle(angle: f32) -> f32 {
    let mut result = angle % (2.0 * std::f32::consts::PI);
    if result < 0.0 {
        result += 2.0 * std::f32::consts::PI;
    }
    result
}
