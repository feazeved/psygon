use macroquad::prelude::*;

pub struct HealthBar {
    position: Vec2,
    size: Vec2,
}

impl HealthBar {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    pub fn draw(&self, current: i32, max: i32) {
        let health_percent: f32 = current as f32 / max as f32;
        let color = if health_percent >= 0.66 {
            GREEN
        } else if health_percent >= 0.33 {
            ORANGE
        } else {
            RED
        };
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x * health_percent,
            self.size.y,
            color,
        );
        draw_rectangle(
            self.position.x + (self.size.x * health_percent),
            self.position.y,
            self.size.x - (self.size.x * health_percent),
            self.size.y,
            DARKBROWN,
        );
        draw_rectangle_lines(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            10.0,
            BLACK,
        );
    }
}

// HOW TESTS WORK
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce() {
        let mut bar = HealthBar::new(Vec2::ZERO, RED, 100, 10);
        bar.increase(101);
        assert_eq!(bar.current, 100);
    }
}
*/
