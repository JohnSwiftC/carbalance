pub mod route;
pub mod storage;

use bevy::prelude::*;
use route::{Map, Stretch};
mod road_render;
use road_render::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RoadRenderPlugin)
        .insert_resource(make_demo_graph())
        .run();
}

fn make_demo_graph() -> RoadRenderGraph {
    RoadRenderGraph {
        junctions: vec![
            Junction {
                pos: Vec2::new(-200.0, 0.0),
            },
            Junction {
                pos: Vec2::new(0.0, 120.0),
            },
            Junction {
                pos: Vec2::new(200.0, 0.0),
            },
        ],
        roads: vec![
            RoadGeom {
                road_id: 0,
                junctions: vec![0, 1],
            },
            RoadGeom {
                road_id: 5,
                junctions: vec![1, 2],
            },
        ],
    }
}

pub fn build_sample_map() -> Map {
    let mut map = Map::with_capacity(15);

    // 0..=14 stretches
    // Chosen so some nodes are "cheap" (fast/low congestion) and some are "expensive".
    map.push(Stretch {
        speed: 12,
        length: 12,
        cars: 3,
        cap: 12,
    }); // 0
    map.push(Stretch {
        speed: 10,
        length: 15,
        cars: 4,
        cap: 12,
    }); // 1
    map.push(Stretch {
        speed: 8,
        length: 16,
        cars: 7,
        cap: 8,
    }); // 2 (expensive)
    map.push(Stretch {
        speed: 15,
        length: 12,
        cars: 2,
        cap: 12,
    }); // 3
    map.push(Stretch {
        speed: 20,
        length: 10,
        cars: 1,
        cap: 10,
    }); // 4
    map.push(Stretch {
        speed: 12,
        length: 18,
        cars: 4,
        cap: 12,
    }); // 5
    map.push(Stretch {
        speed: 10,
        length: 10,
        cars: 9,
        cap: 10,
    }); // 6 (kind of expensive)
    map.push(Stretch {
        speed: 18,
        length: 9,
        cars: 2,
        cap: 12,
    }); // 7
    map.push(Stretch {
        speed: 16,
        length: 16,
        cars: 3,
        cap: 16,
    }); // 8
    map.push(Stretch {
        speed: 14,
        length: 14,
        cars: 5,
        cap: 14,
    }); // 9
    map.push(Stretch {
        speed: 22,
        length: 11,
        cars: 2,
        cap: 22,
    }); // 10 (cheap)
    map.push(Stretch {
        speed: 8,
        length: 24,
        cars: 8,
        cap: 8,
    }); // 11 (isolated component, expensive)
    map.push(Stretch {
        speed: 25,
        length: 10,
        cars: 1,
        cap: 25,
    }); // 12
    map.push(Stretch {
        speed: 12,
        length: 12,
        cars: 6,
        cap: 12,
    }); // 13
    map.push(Stretch {
        speed: 30,
        length: 15,
        cars: 1,
        cap: 30,
    }); // 14 (end)

    // Main component (0..=10 plus 14):
    // Route A spine
    map.connect(0, 1);
    map.connect(1, 2);
    map.connect(2, 3);
    map.connect(3, 4);
    map.connect(4, 14);

    // Route B spine
    map.connect(0, 5);
    map.connect(5, 6);
    map.connect(6, 7);
    map.connect(7, 8);
    map.connect(8, 14);

    // Cross-links (creates many alternate routes)
    map.connect(1, 5);
    map.connect(2, 6);
    map.connect(3, 7);
    map.connect(4, 8);

    // Cycle/shortcut area
    map.connect(5, 9);
    map.connect(9, 7);
    map.connect(9, 10);
    map.connect(10, 8);

    // Isolated component (unreachable from node 0)
    map.connect(11, 12);
    map.connect(12, 13);

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage;

    #[test]
    fn dijkstra_expected_paths() {
        let map = build_sample_map();

        assert_eq!(map.solve(0, 14), vec![0, 5, 9, 10, 8, 14]);
        assert_eq!(map.solve(0, 4), vec![0, 5, 9, 7, 3, 4]);
        assert_eq!(map.solve(2, 14), vec![2, 3, 4, 14]);

        assert_eq!(map.solve(0, 12), Vec::<usize>::new());
        assert_eq!(map.solve(11, 13), vec![11, 12, 13]);

        assert_eq!(map.solve(7, 7), vec![7]);
    }

    #[test]
    fn round_trip_save_load_keeps_solution() {
        let map = build_sample_map();

        map.write_to_file("sample_test.map").unwrap();
        let loaded = storage::load_map("sample_test.map").unwrap();

        assert_eq!(loaded.solve(0, 14), vec![0, 5, 9, 10, 8, 14]);
        assert_eq!(loaded.solve(0, 12), Vec::<usize>::new());
        assert_eq!(loaded.solve(11, 13), vec![11, 12, 13]);
    }
}
