use static_collections::{HashMap, PriorityQueue, Queue};

use crate::{
    behavior::ManualBehavior,
    event::{BehaviorTimeoutEvent, Event, TimerEvent},
};

pub const MAX_HELD_BEHAVIORS: usize = 20;
const HELD_BEH_BACK_ARR_LEN: usize = MAX_HELD_BEHAVIORS * 2;
pub const MAX_EVENTS: usize = 100;
pub const MAX_TIMER_EVENTS: usize = 50;

pub struct State {
    // The number of elements in the HashMap's backing array
    held_behaviors: HashMap<usize, ManualBehavior, HELD_BEH_BACK_ARR_LEN>,
    event_queue: Queue<Event, MAX_EVENTS>,
    timer_events: PriorityQueue<TimerEvent, MAX_TIMER_EVENTS>,
}
