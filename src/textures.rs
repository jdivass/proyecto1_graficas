use crate::maze::Maze;
use raylib::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

pub struct TextureManager {
    wall_textures: HashMap<(usize, usize, usize), WallTexture>,
    default_texture: WallTexture,
    final_texture: WallTexture,
    sky_texture: WallTexture,
    floor_texture: WallTexture,
    final_wall: (usize, usize),
}

pub struct WallTexture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl WallTexture {
    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = (u.clamp(0.0, 1.0) * (self.width - 1) as f32) as usize;
        let y = (v.clamp(0.0, 1.0) * (self.height - 1) as f32) as usize;
        self.pixels[y * self.width + x]
    }
}

impl TextureManager {
    pub fn new(maze: &Maze) -> Result<Self, String> {
        let texture_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("textures");
        let background = load_image(&texture_dir.join("background.png"))?;
        let final_image = load_image(&texture_dir.join("final.png"))?;
        let sky_image = load_image(&texture_dir.join("sky.png"))?;
        let floor_image =
            load_first_image(&[texture_dir.join("floor.png"), texture_dir.join("floor.jpg")])?;

        let mut artwork_paths: Vec<PathBuf> = fs::read_dir(&texture_dir)
            .map_err(|error| format!("Failed to read {}: {error}", texture_dir.display()))?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension().and_then(|extension| extension.to_str()) == Some("png")
                    && !matches!(
                        path.file_name().and_then(|name| name.to_str()),
                        Some("background.png" | "final.png" | "sky.png" | "floor.png")
                    )
            })
            .collect();
        artwork_paths.sort();

        let artwork = artwork_paths
            .iter()
            .map(|path| load_image(path))
            .collect::<Result<Vec<_>, _>>()?;

        if artwork.is_empty() {
            return Err("No museum artwork was found in textures/".to_string());
        }

        let exhibit_by_face = assign_exhibits(maze, artwork.len());
        let wall_textures = exhibit_by_face
            .iter()
            .map(|(face, artwork_index)| {
                (
                    *face,
                    compose_wall(&background, &artwork[*artwork_index], 0.24),
                )
            })
            .collect();
        let default_texture = compose_wall(&background, &artwork[0], 0.24);
        let final_texture = compose_wall(&background, &final_image, 0.16);
        let sky_texture = cache_image(&sky_image);
        let floor_texture = cache_image(&floor_image);

        Ok(Self {
            wall_textures,
            default_texture,
            final_texture,
            sky_texture,
            floor_texture,
            final_wall: find_final_wall(maze),
        })
    }

    pub fn wall_texture(&self, map_x: usize, map_y: usize, wall_side: usize) -> &WallTexture {
        if (map_x, map_y) == self.final_wall {
            &self.final_texture
        } else {
            self.wall_textures
                .get(&(map_x, map_y, wall_side))
                .unwrap_or(&self.default_texture)
        }
    }

    pub fn sky_pixel(&self, angle: f32, v: f32) -> Color {
        let u = (angle / std::f32::consts::TAU).rem_euclid(1.0);
        self.sky_texture.sample(u, v)
    }

    pub fn floor_pixel(
        &self,
        world_x: f32,
        world_y: f32,
        maze_width: f32,
        maze_height: f32,
    ) -> Color {
        // Map one copy of floor.jpg across the complete maze instead of every cell.
        self.floor_texture
            .sample(world_x / maze_width, world_y / maze_height)
    }
}

fn cache_image(image: &Image) -> WallTexture {
    let width = image.width as usize;
    let height = image.height as usize;
    let mut pixels = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            pixels.push(image.get_color(x as i32, y as i32));
        }
    }

    WallTexture {
        width,
        height,
        pixels,
    }
}

fn compose_wall(background: &Image, exhibit: &Image, margin: f32) -> WallTexture {
    let width = background.width as usize;
    let height = background.height as usize;
    let mut pixels = Vec::with_capacity(width * height);

    for y in 0..height {
        let v = y as f32 / (height - 1) as f32;
        for x in 0..width {
            let u = x as f32 / (width - 1) as f32;
            let background_color = background.get_color(x as i32, y as i32);

            if u < margin || u > 1.0 - margin || v < margin || v > 1.0 - margin {
                pixels.push(background_color);
                continue;
            }

            let exhibit_u = (u - margin) / (1.0 - margin * 2.0);
            let exhibit_v = (v - margin) / (1.0 - margin * 2.0);
            pixels.push(alpha_blend(
                background_color,
                sample_image(exhibit, exhibit_u, exhibit_v),
            ));
        }
    }

    WallTexture {
        width,
        height,
        pixels,
    }
}

fn assign_exhibits(maze: &Maze, artwork_count: usize) -> HashMap<(usize, usize, usize), usize> {
    let mut assignments = HashMap::new();
    let mut next_artwork = 0;

    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '#' {
                continue;
            }

            for (dx, dy, wall_side) in [(1isize, 0isize, 0usize), (-1, 0, 0), (0, 1, 1), (0, -1, 1)]
            {
                let wall_x = x as isize + dx;
                let wall_y = y as isize + dy;
                if wall_x < 0 || wall_y < 0 {
                    continue;
                }

                let face = (wall_x as usize, wall_y as usize, wall_side);
                if face.1 < maze.len()
                    && face.0 < maze[face.1].len()
                    && maze[face.1][face.0] == '#'
                    && !assignments.contains_key(&face)
                {
                    assignments.insert(face, next_artwork % artwork_count);
                    next_artwork += 1;
                }
            }
        }
    }

    assignments
}

fn load_image(path: &Path) -> Result<Image, String> {
    let path_text = path
        .to_str()
        .ok_or_else(|| format!("Invalid texture path: {}", path.display()))?;
    Image::load_image(path_text)
        .map_err(|error| format!("Failed to load {}: {error}", path.display()))
}

fn load_first_image(paths: &[PathBuf]) -> Result<Image, String> {
    let mut errors = Vec::new();

    for path in paths {
        if !path.exists() {
            continue;
        }

        match load_image(path) {
            Ok(image) => return Ok(image),
            Err(error) => errors.push(error),
        }
    }

    if errors.is_empty() {
        Err(format!(
            "None of these texture files exist: {}",
            paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))
    } else {
        Err(errors.join("; "))
    }
}

fn sample_image(image: &Image, u: f32, v: f32) -> Color {
    let x = (u.clamp(0.0, 1.0) * (image.width - 1) as f32) as i32;
    let y = (v.clamp(0.0, 1.0) * (image.height - 1) as f32) as i32;
    image.get_color(x, y)
}

fn alpha_blend(background: Color, foreground: Color) -> Color {
    let alpha = foreground.a as f32 / 255.0;
    let blend = |back: u8, front: u8| (front as f32 * alpha + back as f32 * (1.0 - alpha)) as u8;

    Color::new(
        blend(background.r, foreground.r),
        blend(background.g, foreground.g),
        blend(background.b, foreground.b),
        255,
    )
}

fn find_final_wall(maze: &Maze) -> (usize, usize) {
    let start = find_cell(maze, 's').unwrap_or((1, 1));
    let finish = find_cell(maze, 'f').unwrap_or((maze[0].len() - 2, maze.len() - 2));
    let width = maze[0].len();
    let mut queue = VecDeque::from([start]);
    let mut visited = vec![vec![false; width]; maze.len()];
    let mut previous = vec![vec![None; width]; maze.len()];
    visited[start.1][start.0] = true;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == finish {
            break;
        }

        for (dx, dy) in [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)] {
            let next_x = x as isize + dx;
            let next_y = y as isize + dy;
            if next_x < 0 || next_y < 0 {
                continue;
            }

            let next = (next_x as usize, next_y as usize);
            if next.1 < maze.len()
                && next.0 < width
                && !visited[next.1][next.0]
                && maze[next.1][next.0] != '#'
            {
                visited[next.1][next.0] = true;
                previous[next.1][next.0] = Some((x, y));
                queue.push_back(next);
            }
        }
    }

    if let Some(before_finish) = previous[finish.1][finish.0] {
        let wall_x = finish.0 as isize + finish.0 as isize - before_finish.0 as isize;
        let wall_y = finish.1 as isize + finish.1 as isize - before_finish.1 as isize;
        if wall_x >= 0 && wall_y >= 0 {
            let wall = (wall_x as usize, wall_y as usize);
            if wall.1 < maze.len() && wall.0 < width && maze[wall.1][wall.0] == '#' {
                return wall;
            }
        }
    }

    // The finish is in the bottom-right corner, so its outer walls are safe fallbacks.
    (finish.0, finish.1 + 1)
}

fn find_cell(maze: &Maze, target: char) -> Option<(usize, usize)> {
    maze.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|cell| *cell == target).map(|x| (x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_the_environment_texture_files() {
        let maze = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', 's', '.', 'f', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];

        assert!(TextureManager::new(&maze).is_ok());
    }
}
