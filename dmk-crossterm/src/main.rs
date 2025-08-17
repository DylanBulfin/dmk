use std::{
    io::{self, Write, stdout},
    panic::set_hook,
    process::exit,
    thread::sleep,
    time::Duration,
};

use crossterm::{
    cursor::{self, MoveDown, MoveTo, MoveToColumn, RestorePosition, SavePosition},
    event::{KeyCode, poll, read},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown, ScrollUp,
        disable_raw_mode, enable_raw_mode,
    },
};
use dmk::{
    behavior::{
        NoArgBehavior, hold_tap::HoldTap, key_press::KeyPress, momentary_layer::MomentaryLayer,
    },
    key::Key,
    layer::{Layer, MAX_LAYERS},
    physical_layout::{MAX_KEYS, PhysicalLayout},
    state::State,
    timer::Timer,
    vboard::VirtualKeys,
};

#[derive(Debug)]
pub struct CrosstermVirtKeys {
    a: Option<std::time::Instant>,
    r: Option<std::time::Instant>,
    s: Option<std::time::Instant>,
    t: Option<std::time::Instant>,
}
#[derive(Debug)]
pub struct CrosstermPhysKeys {
    a: bool,
    r: bool,
    s: bool,
    t: bool,
}

impl PhysicalLayout for CrosstermPhysKeys {
    fn keys(&self) -> usize {
        4
    }

    fn get_status(&self, key: usize) -> bool {
        match key {
            0 => self.a,
            1 => self.r,
            2 => self.s,
            3 => self.t,
            _ => panic!("Unexpected key query when len = 4: {}", key),
        }
    }

    fn get_arr_copy(&self) -> [bool; MAX_KEYS] {
        let mut base = [false; MAX_KEYS];
        base[0] = self.a;
        base[1] = self.r;
        base[2] = self.s;
        base[3] = self.t;

        base
    }
}

pub struct CrosstermTimer {
    base_instant: std::time::Instant,
}

impl Timer for CrosstermTimer {
    fn microseconds(&self) -> u64 {
        (self.base_instant.elapsed()).as_micros() as u64
    }
}

fn main() -> io::Result<()> {
    set_hook(Box::new(|pi| {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        println!("{}", pi);
    }));

    execute!(
        stdout(),
        SavePosition,
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;

    enable_raw_mode()?;

    let mut virt_keys = CrosstermVirtKeys {
        a: None,
        r: None,
        s: None,
        t: None,
    };

    let phys_keys = CrosstermPhysKeys {
        a: false,
        r: false,
        s: false,
        t: false,
    };

    let timer = CrosstermTimer {
        base_instant: std::time::Instant::now(),
    };
    let mut base_behaviors = [None; MAX_KEYS];

    base_behaviors[0] = Some(MomentaryLayer::new(1).into());
    base_behaviors[1] = Some(KeyPress::new(Key::A).into());
    base_behaviors[2] = Some(KeyPress::new(Key::R).into());
    base_behaviors[3] = Some(
        HoldTap::new(
            KeyPress::new(Key::S).into(),
            KeyPress::new(Key::T).into(),
            dmk::timer::Duration::from_millis(5000),
            true,
        )
        .into(),
    );

    let mut l1_behaviors = [None; MAX_KEYS];

    l1_behaviors[0] = Some(NoArgBehavior::Transparent.into());
    l1_behaviors[1] = Some(KeyPress::new(Key::U).into());
    l1_behaviors[2] = Some(KeyPress::new(Key::V).into());
    l1_behaviors[3] = Some(KeyPress::new(Key::W).into());

    let base_layer = Layer::new(phys_keys.keys(), base_behaviors);
    let layer1 = Layer::new(phys_keys.keys(), l1_behaviors);

    let mut state = State::new([base_layer, layer1], phys_keys, timer);

    loop {
        if poll(Duration::from_millis(150))? {
            match read()? {
                crossterm::event::Event::Key(key_event) => match key_event.code {
                    KeyCode::Char(c) => match c {
                        // 'a' => keys.a = key_event.is_press(),
                        // 'r' => keys.r = key_event.is_press(),
                        // 's' => keys.s = key_event.is_press(),
                        // 't' => keys.t = key_event.is_press(),
                        'a' => {
                            if key_event.is_press() {
                                virt_keys.a = Some(std::time::Instant::now());
                            }
                        }
                        'r' => {
                            if key_event.is_press() {
                                virt_keys.r = Some(std::time::Instant::now());
                            }
                        }
                        's' => {
                            if key_event.is_press() {
                                virt_keys.s = Some(std::time::Instant::now());
                            }
                        }
                        't' => {
                            if key_event.is_press() {
                                virt_keys.t = Some(std::time::Instant::now());
                            }
                        }
                        'q' => break,
                        // _ => println!("{:?}", key_event),
                        _ => {}
                    },
                    // _ => println!("{:?}", key_event),
                    _ => {}
                },
                _ => {}
            }
        }

        state.get_phys_layout_mut().a = match virt_keys.a {
            Some(inst) => {
                if inst.elapsed().as_millis() >= 1500 {
                    false
                } else {
                    true
                }
            }
            None => false,
        };
        state.get_phys_layout_mut().r = match virt_keys.r {
            Some(inst) => {
                if inst.elapsed().as_millis() >= 1500 {
                    false
                } else {
                    true
                }
            }
            None => false,
        };
        state.get_phys_layout_mut().s = match virt_keys.s {
            Some(inst) => {
                if inst.elapsed().as_millis() >= 1500 {
                    false
                } else {
                    true
                }
            }
            None => false,
        };
        state.get_phys_layout_mut().t = match virt_keys.t {
            Some(inst) => {
                if inst.elapsed().as_millis() >= 1500 {
                    false
                } else {
                    true
                }
            }
            None => false,
        };
        state.main_iteration();

        let VirtualKeys {
            a,
            r,
            s,
            t,
            u,
            v,
            w,
            ..
        } = state.get_vboard().keys;

        execute!(stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        stdout().write_fmt(format_args!("{} {} {} {} {} {} {}", a, r, s, t, u, v, w))?;
        stdout().flush()?;
    }

    execute!(stdout(), LeaveAlternateScreen, RestorePosition)?;

    disable_raw_mode()?;

    Ok(())
}
