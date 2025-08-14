use crate::behavior::KeyState;

pub enum AltBehavior<I, BF, AF>
where
    I: Iterator<Item = Event>,
    BF: FnOnce(&KeyState) -> I,
{
    Standard(StandardBehavior<I, BF, AF>),
    EventDriven(EventDrivenBehavior<I, BF, AF>),
}

pub struct StandardBehavior<I, BF, AF>
where
    I: Iterator<Item = Event>,
    BF: FnOnce(&KeyState) -> I,
{
    before: BF,
    after: AF,
}

pub struct EventDrivenBehavior<I, BF, AF>
where
    I: Iterator<Item = Event>,
    BF: FnOnce(&KeyState) -> I,
{
    before: BF,
    callback: AF,
}

pub struct Event {}
