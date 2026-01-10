use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Junction {
    pub pos: Vec2,
}

#[derive(Clone, Debug)]
pub struct RoadGeom {
    pub road_id: usize,
    pub junctions: Vec<usize>,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct RoadRenderGraph {
    pub junctions: Vec<Junction>,
    pub roads: Vec<RoadGeom>,
}

#[derive(Resource, Clone, Debug)]
pub struct RoadRenderStyle {
    pub road_color: Color,
    pub junction_color: Color,
    pub junction_radius: f32,
    pub z: f32,
}

impl Default for RoadRenderStyle {
    fn default() -> Self {
        Self {
            road_color: Color::WHITE,
            junction_color: Color::srgb(0.9, 0.9, 1.0),
            junction_radius: 4.0,
            z: 0.0,
        }
    }
}

pub struct RoadRenderPlugin;

impl Plugin for RoadRenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadRenderStyle>()
            .add_systems(Startup, spawn_2d_camera_if_missing)
            .add_systems(Update, draw_roads_gizmos);
    }
}

fn spawn_2d_camera_if_missing(mut commands: Commands, q: Query<(), With<Camera2d>>) {
    if q.is_empty() {
        commands.spawn(Camera2d);
    }
}

fn draw_roads_gizmos(
    graph: Option<Res<RoadRenderGraph>>,
    style: Res<RoadRenderStyle>,
    mut gizmos: Gizmos,
) {
    let Some(graph) = graph else {
        return;
    };

    for j in &graph.junctions {
        gizmos.circle_2d(j.pos, style.junction_radius, style.junction_color);
    }

    for road in &graph.roads {
        if road.junctions.len() < 2 {
            continue;
        }
        let a_idx = road.junctions[0];
        let a = graph.junctions[a_idx].pos;

        for &b_idx in &road.junctions[1..] {
            let b = graph.junctions[b_idx].pos;
            gizmos.line_2d(a, b, style.road_color);
        }
    }
}
