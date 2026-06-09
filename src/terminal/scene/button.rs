#[derive(Debug)]
pub struct Button {
    pub content_id: usize,
    // index of the potential next scene
    pub next_scene_id: isize,
}
impl Button {
    pub fn new(content_id: usize, next_scene_id: isize) -> Self {
        Button {
            content_id,
            next_scene_id
        }
    }
    pub fn clone(&self) -> Self {
        Button {
            content_id: self.content_id,
            next_scene_id: self.next_scene_id
        }
    }
}