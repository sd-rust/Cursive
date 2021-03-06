//! Dummy backend
use backend;
use event;
use theme;
use vec::Vec2;

use std::time::Duration;

use chan;

pub struct Backend;

impl Backend {
    pub fn init() -> Box<backend::Backend>
    where
        Self: Sized,
    {
        Box::new(Backend)
    }
}

impl backend::Backend for Backend {
    fn finish(&mut self) {}

    fn refresh(&mut self) {}

    fn has_colors(&self) -> bool {
        false
    }

    fn screen_size(&self) -> Vec2 {
        (1, 1).into()
    }

    fn prepare_input(
        &mut self, event_sink: &chan::Sender<event::Event>, _timeout: Duration,
    ) {
        event_sink.send(event::Event::Exit)
    }

    fn print_at(&self, _: Vec2, _: &str) {}

    fn clear(&self, _: theme::Color) {}

    // This sets the Colours and returns the previous colours
    // to allow you to set them back when you're done.
    fn set_color(&self, colors: theme::ColorPair) -> theme::ColorPair {
        colors
    }

    fn set_effect(&self, _: theme::Effect) {}
    fn unset_effect(&self, _: theme::Effect) {}
}
