use std::io::Write;
use std::{cmp::Ordering, collections::BinaryHeap, fs::File, path::Path};

pub struct Stretch {
    pub speed: u32,
    pub length: u32,
    pub cars: u32,
    pub cap: u32,
}

impl Stretch {
    fn weight(&self) -> f32 {
        let lov = self.length as f32 / self.speed as f32;
        let noc = self.cars as f32 / self.cap as f32;
        let noc4 = noc * noc * noc * noc;
        (0.15 * noc4 + 1.0) * lov
    }
}

#[derive(Clone, Copy, Debug)]
struct F32Ord(f32);

impl PartialEq for F32Ord {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}
impl Eq for F32Ord {}

impl PartialOrd for F32Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F32Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

pub struct Map {
    stretches: Vec<Stretch>,
    adj: Vec<Vec<usize>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            stretches: Vec::new(),
            adj: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            stretches: Vec::with_capacity(cap),
            adj: Vec::new(),
        }
    }

    pub fn push(&mut self, stretch: Stretch) {
        self.stretches.push(stretch);
        self.adj.push(Vec::new());
    }

    pub fn connect(&mut self, one: u32, two: u32) {
        self.adj[one as usize].push(two as usize);
        self.adj[two as usize].push(one as usize);
    }

    pub fn solve(&self, start: usize, end: usize) -> Vec<usize> {
        if start >= self.stretches.len() || end >= self.stretches.len() {
            return Vec::new();
        }

        let n = self.stretches.len();
        let mut dist = vec![f32::INFINITY; n];
        let mut prev = vec![None::<usize>; n];

        let mut heap = BinaryHeap::<(std::cmp::Reverse<F32Ord>, usize)>::new();

        dist[start] = self.stretches[start].weight();
        heap.push((std::cmp::Reverse(F32Ord(dist[start])), start));

        while let Some((std::cmp::Reverse(F32Ord(d)), u)) = heap.pop() {
            if d != dist[u] {
                continue;
            }
            if u == end {
                break;
            }

            for &v in &self.adj[u] {
                let alt = d + self.stretches[v].weight();
                if alt < dist[v] {
                    dist[v] = alt;
                    prev[v] = Some(u);
                    heap.push((std::cmp::Reverse(F32Ord(alt)), v));
                }
            }
        }

        if !dist[end].is_finite() {
            return Vec::new();
        }

        let mut path = Vec::new();
        let mut cur = end;
        path.push(cur);
        while cur != start {
            if let Some(p) = prev[cur] {
                cur = p;
                path.push(cur);
            } else {
                return Vec::new();
            }
        }
        path.reverse();
        path
    }

    pub fn write_to_file<T: AsRef<Path>>(&self, name: T) -> std::io::Result<()> {
        let mut file = File::create(name)?;

        for stretch in &self.stretches {
            file.write_all(&[0])?;
            file.write_all(&stretch.speed.to_le_bytes())?;
            file.write_all(&stretch.length.to_le_bytes())?;
            file.write_all(&stretch.cars.to_le_bytes())?;
            file.write_all(&stretch.cap.to_le_bytes())?;
            file.write_all(&[b';'])?;
        }

        for (i, adj) in self.adj.iter().enumerate() {
            file.write_all(&[1])?;
            todo!();
            // TODO:
            // rework loading so it just uses the predefined adjacency array
        }

        Ok(())
    }
}
