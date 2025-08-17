use crate::{
    behavior::Behavior,
    evec,
    event::{EVec, Event, LayerEvent},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MomentaryLayer {
    layer: usize,
}

impl Behavior for MomentaryLayer {
    fn on_press(&mut self, _ks: &super::KeyState) -> EVec {
        evec![Event::LayerEvent(LayerEvent::AddLayer(self.layer))]
    }

    fn on_release(&mut self, _ks: &super::KeyState) -> EVec {
        evec![Event::LayerEvent(LayerEvent::RemoveDownToLayer(self.layer))]
    }

    fn try_get_delay(&self) -> Option<crate::timer::Duration> {
        None
    }

    fn after_delay(&mut self, _ks: &super::KeyState) -> EVec {
        evec![]
    }
}
