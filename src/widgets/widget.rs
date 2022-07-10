pub trait Widget {
    fn get_pos(&self) -> (i32, i32);
    fn get_size(&self) -> (u32, u32);
    fn get_texture(&self) -> String;
    fn mouse_down(&mut self);
    fn mouse_up(&self);
}