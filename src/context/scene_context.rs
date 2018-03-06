use std::net::{UdpSocket};
use std::rc::Rc;
use std::cell::RefCell;
use sdl2::event::Event;

pub type RefSceneContext = Rc<RefCell<SceneContext>>;

pub trait SceneContext {
    fn render(&self);
    fn update(&mut self, network: &UdpSocket);
    fn user_input(&mut self, event: Event, network: &UdpSocket);
    fn switch_context(&self) -> Option<RefSceneContext>;
}
