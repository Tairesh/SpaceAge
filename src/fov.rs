use geometry::Point;
use std::collections::HashSet;

// copy/pasted from https://github.com/amethyst/bracket-lib/blob/master/bracket-pathfinding/src/field_of_view/recursive_shadowcasting.rs

pub trait FovMap {
    fn point2d_to_index(&self, pt: Point) -> usize {
        pt.to_index(self.dimensions().x).unwrap()
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        Point::from_index(idx, self.dimensions().x)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.to_index(self.dimensions().x).is_some()
    }

    fn dimensions(&self) -> Point;
    fn is_opaque(&self, _idx: usize) -> bool;
}

pub fn field_of_view_set(center: Point, range: u32, fov_check: &dyn FovMap) -> HashSet<Point> {
    let mut visible_points: HashSet<Point> =
        HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

    visible_points.insert(center);

    /* N, NE, E, SE, S, SW, W, NW */
    const SECTORS: [(i32, i32); 8] = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let r2 = range * range;
    let dimensions = fov_check.dimensions();

    // Add visibility for every 45 degree line:
    let mut visibility_per_sector = [false; 8];
    for (i, (dx, dy)) in SECTORS.iter().enumerate() {
        let mut current = center;
        loop {
            current = Point::new(current.x + dx, current.y + dy);
            if current.x < 0
                || current.x >= dimensions.x
                || current.y < 0
                || current.y >= dimensions.y
            {
                break;
            }
            let x2 = current.x - center.x;
            let x2 = x2 * x2;
            let y2 = current.y - center.y;
            let y2 = y2 * y2;
            if x2 + y2 > r2 as i32 {
                break;
            }

            let idx = fov_check.point2d_to_index(current);
            visible_points.insert(current);
            if fov_check.is_opaque(idx) {
                break;
            }
            visibility_per_sector[i] = true;
        }
    }

    let mut scanner = ScanFovData {
        center,
        dimensions,
        range_2: r2,
        fov_check,
        visible_points: &mut visible_points,
    };
    if visibility_per_sector[0] {
        scanner.scan_N2NW(1, 0., 1.);
        scanner.scan_N2NE(1, 0., 1.);
    }

    if visibility_per_sector[2] {
        scanner.scan_E2NE(1, 0., 1.);
        scanner.scan_E2SE(1, 0., 1.);
    }

    if visibility_per_sector[4] {
        scanner.scan_S2SE(1, 0., 1.);
        scanner.scan_S2SW(1, 0., 1.);
    }

    if visibility_per_sector[6] {
        scanner.scan_W2SW(1, 0., 1.);
        scanner.scan_W2NW(1, 0., 1.);
    }

    visible_points
        .iter()
        .copied()
        .filter(|p| fov_check.in_bounds(*p))
        .collect()
}

struct ScanFovData<'a> {
    center: Point,
    dimensions: Point,
    range_2: u32,
    fov_check: &'a dyn FovMap,
    visible_points: &'a mut HashSet<Point>,
}

#[allow(non_snake_case)]
impl ScanFovData<'_> {
    fn is_transparent(&self, idx: usize, point: Point) -> bool {
        if self.fov_check.in_bounds(point) {
            !self.fov_check.is_opaque(idx)
        } else {
            false
        }
    }

    fn distance_to_center(&self, point: Point) -> u32 {
        point.square_distance_to(self.center)
    }

    fn insert_visible_for_vertical(&mut self, point: Point) -> bool {
        let idx = self.fov_check.point2d_to_index(point);
        let mut is_visible = self.is_transparent(idx, point);

        if self.distance_to_center(point) <= self.range_2 {
            if point.x != self.center.x {
                self.visible_points.insert(point);
            }
        } else {
            is_visible = false;
        }
        is_visible
    }

    fn insert_visible_for_horizontal(&mut self, point: Point) -> bool {
        let idx = self.fov_check.point2d_to_index(point);
        let mut is_visible = self.is_transparent(idx, point);

        if self.distance_to_center(point) <= self.range_2 {
            if self.center.y != point.y {
                self.visible_points.insert(point);
            }
        } else {
            is_visible = false;
        }
        is_visible
    }

    fn scan_N2NE(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y - distance as i32);
        if current.y < 0 {
            return;
        }

        current.x = self.center.x + (start_slope * distance as f32 + 0.5) as i32;
        if current.x >= self.dimensions.x {
            return;
        }

        let mut end_x = self.center.x + (end_slope * distance as f32 + 0.5) as i32;
        if end_x >= self.dimensions.x {
            end_x = self.dimensions.x - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_x in current.x..=end_x {
            current.x = current_x;

            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_N2NE(
                    distance + 1,
                    start_slope,
                    ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_N2NE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_N2NW(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y - distance as i32);
        if current.y < 0 {
            return;
        }

        current.x = self.center.x - (start_slope * distance as f32 + 0.5) as i32;
        if current.x < 0 {
            return;
        }

        let mut end_x = self.center.x - (end_slope * distance as f32 + 0.5) as i32;
        if end_x < 0 {
            end_x = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.x >= end_x {
            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_N2NW(
                    distance + 1,
                    start_slope,
                    ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.x -= 1;
        }
        if last_visible {
            self.scan_N2NW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_S2SE(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y + distance as i32);
        if current.y >= self.dimensions.y {
            return;
        }

        current.x = self.center.x + (start_slope * distance as f32 + 0.5) as i32;
        if current.x >= self.dimensions.x {
            return;
        }

        let mut end_x = self.center.x + (end_slope * distance as f32 + 0.5) as i32;
        if end_x >= self.dimensions.x {
            end_x = self.dimensions.x - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_x in current.x..=end_x {
            current.x = current_x;

            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_S2SE(
                    distance + 1,
                    start_slope,
                    ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_S2SE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_S2SW(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y + distance as i32);
        if current.y >= self.dimensions.y {
            return;
        }

        current.x = self.center.x - (start_slope * distance as f32 + 0.5) as i32;
        if current.x < 0 {
            return;
        }

        let mut end_x = self.center.x - (end_slope * distance as f32 + 0.5) as i32;
        if end_x < 0 {
            end_x = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.x >= end_x {
            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_S2SW(
                    distance + 1,
                    start_slope,
                    ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.x -= 1;
        }
        if last_visible {
            self.scan_S2SW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_E2SE(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x + distance as i32, 0);
        if current.x >= self.dimensions.x {
            return;
        }

        current.y = self.center.y + (start_slope * distance as f32 + 0.5) as i32;
        if current.y >= self.dimensions.y {
            return;
        }

        let mut end_y = self.center.y + (end_slope * distance as f32 + 0.5) as i32;
        if end_y >= self.dimensions.y {
            end_y = self.dimensions.y - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_y in current.y..=end_y {
            current.y = current_y;

            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_E2SE(
                    distance + 1,
                    start_slope,
                    ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_E2SE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_E2NE(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x + distance as i32, 0);
        if current.x >= self.dimensions.x {
            return;
        }

        current.y = self.center.y - (start_slope * distance as f32 + 0.5) as i32;
        if current.y < 0 {
            return;
        }

        let mut end_y = self.center.y - (end_slope * distance as f32 + 0.5) as i32;
        if end_y < 0 {
            end_y = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.y >= end_y {
            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_E2NE(
                    distance + 1,
                    start_slope,
                    ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.y -= 1;
        }
        if last_visible {
            self.scan_E2NE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_W2SW(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x - distance as i32, 0);
        if current.x < 0 {
            return;
        }

        current.y = self.center.y + (start_slope * distance as f32 + 0.5) as i32;
        if current.y >= self.dimensions.y {
            return;
        }

        let mut end_y = self.center.y + (end_slope * distance as f32 + 0.5) as i32;
        if end_y >= self.dimensions.y {
            end_y = self.dimensions.y - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_y in current.y..=end_y {
            current.y = current_y;

            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_W2SW(
                    distance + 1,
                    start_slope,
                    ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_W2SW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_W2NW(&mut self, distance: u32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x - distance as i32, 0);
        if current.x < 0 {
            return;
        }

        current.y = self.center.y - (start_slope * distance as f32 + 0.5) as i32;
        if current.y < 0 {
            return;
        }

        let mut end_y = self.center.y - (end_slope * distance as f32 + 0.5) as i32;
        if end_y < 0 {
            end_y = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.y >= end_y {
            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_W2NW(
                    distance + 1,
                    start_slope,
                    ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.y -= 1;
        }
        if last_visible {
            self.scan_W2NW(distance + 1, start_slope, end_slope);
        }
    }
}
