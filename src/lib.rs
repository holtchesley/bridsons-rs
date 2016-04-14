#![feature(test)]


extern crate rand;
extern crate test;


pub mod bridsons {

    use rand::Rng;
    use std::f32;
    use std::collections::HashMap;
    pub const SQRT2: f32 = 1.414213562373095048;

    #[derive(Clone, Debug)]
    pub struct Point2d {
        x: f32,
        y: f32
    }

    impl Point2d {

        pub fn rand_in_rect(width:f32, height:f32, seed: &mut Rng) -> Point2d {
            Point2d{x: seed.next_f32()* width, y: seed.next_f32()*height}
        }

        pub fn d(&self,other: &Point2d) -> f32{
            let xd = other.x - self.x;
            let yd = other.y - self.y;
            xd.hypot(yd)
        }

        pub fn rand_in_annulus(&self, r: f32, rand: &mut Rng) -> Point2d {
            let radius = rand.next_f32().mul_add(r,r);
            let (xd,yd) = (rand.next_f32() * f32::consts::PI).sin_cos();
            Point2d{x:xd*radius+ self.x, y:yd*radius+self.y}
        }

        pub fn in_box(&self, other: &Point2d) -> bool {
            (self.x < other.x) && (self.y < other.y) && (self.x > 0.0) && (self.y > 0.0)
        }

        pub fn within_r(&self, r: f32, other: &Point2d) -> bool {
            self.d(other) < r
        }

    }


    pub struct Grid2d {
        grid_size: f32,
        grid: HashMap<(u32,u32),Point2d>
    }

    impl Grid2d {
        pub fn from_radius(radius:f32) -> Grid2d {
            let grid_size = radius / SQRT2;
            Grid2d{grid_size: grid_size, grid: HashMap::new()}
        }

        fn grid_coord(&self, point: &Point2d) -> (u32, u32){
            ((point.x / self.grid_size).floor() as u32,(point.y / self.grid_size).floor() as u32)
        }

        pub fn has_nearby(&self, r: f32, point: &Point2d) -> bool {
            fn buf_dec(a:u32) -> u32 {
                if a > 1 {a-1} else {0}
            }
            let (cx,cy) = self.grid_coord(point);
            for xx in buf_dec(cx)..(cx+2) {
                for yy in buf_dec(cy)..(cy+2) {
                    if let Some(pt) = self.grid.get(&(xx,yy)) {
                        if pt.within_r(r, point) {
                            return true
                        }
                    }
                }
            }
            false
        }

        pub fn add_point(&mut self, point: Point2d) {
            let gc = self.grid_coord(&point);
            self.grid.insert(gc,point);
        }
    }

    pub fn bridsons_method_2d(width: f32, height: f32, radius: f32, attempts:u32, seed: &mut Rng) -> Vec<Point2d> {
        let mut active_set = vec![];
        let mut inactive_set = vec![];
        let mut grid = Grid2d::from_radius(radius);
        let ur_corner = Point2d{x:width, y:height};
        let pt = Point2d::rand_in_rect(width, height, seed);
        active_set.push(pt.to_owned());
        grid.add_point(pt);

        while !active_set.is_empty() {
            let redundant_len_because_rust_is_silly = (seed.next_u32() as usize) % active_set.len();
            let active_point = active_set.swap_remove(redundant_len_because_rust_is_silly);
            if let Some(found_pt) = (0..attempts).map(|_| active_point.rand_in_annulus(radius, seed)).find(|pt| pt.in_box(&ur_corner) && !grid.has_nearby(radius,pt)) {
                active_set.push(active_point);
                active_set.push(found_pt.to_owned());
                grid.add_point(found_pt);
            } else {
                inactive_set.push(active_point);
            }

        }

        inactive_set
    }

    #[cfg(test)]
    mod tests {
        extern crate rand;

        use rand::{XorShiftRng, SeedableRng};
        use super::bridsons_method_2d;
        use test::Bencher;

        #[test]
        fn it_works() {
            bridsons_method_2d(10.0, 10.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3]));
        }

        #[test]
        fn it_works_count_10() {
            println!("Count: {}", bridsons_method_2d(10.0, 10.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3])).len());
        }

        #[test]
        fn it_works_count_100() {
            println!("Count: {}", bridsons_method_2d(100.0, 100.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 61234, 34342342, 4444443])).len());
        }

        #[test]
        fn it_works_count_512() {
            println!("Count: {}", bridsons_method_2d(512.0, 512.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3])).len());
        }


        #[bench]
        fn bench_bridsons_008(b: &mut Bencher) {
            b.iter(|| bridsons_method_2d(8.0, 8.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3])));
        }

        #[bench]
        fn bench_bridsons_016(b: &mut Bencher) {
            b.iter(|| bridsons_method_2d(16.0, 16.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3])));
        }

        #[bench]
        fn bench_bridsons_032(b: &mut Bencher) {
            b.iter(|| bridsons_method_2d(32.0, 32.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3])));
        }

        #[bench]
        fn bench_bridsons_064(b: &mut Bencher) {
            b.iter(|| bridsons_method_2d(64.0, 64.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3])));
        }

        #[bench]
        fn bench_bridsons_128(b: &mut Bencher) {
            b.iter(|| bridsons_method_2d(128.0, 128.0, 2.0, 30, &mut XorShiftRng::from_seed([4, 6, 2, 3])));
        }

    }
}



