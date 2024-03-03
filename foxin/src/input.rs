use bevy::{
    app::{App, First},
    ecs::{
        event::EventWriter,
        system::{ResMut, Resource},
    },
    math::U16Vec2,
};
use crossterm::{
    event::{
        Event, poll, read,
        KeyEventKind,
        EnableFocusChange, DisableFocusChange,
        EnableMouseCapture, DisableMouseCapture,
        EnableBracketedPaste, DisableBracketedPaste,
    },
    ExecutableCommand,
};
use std::{time::Duration, io::stdout};

pub use crossterm::event::{KeyCode, KeyModifiers};

pub(crate) fn build(app: &mut App) {
    stdout().execute(EnableFocusChange).unwrap();
    stdout().execute(EnableMouseCapture).unwrap();
    stdout().execute(EnableBracketedPaste).unwrap();
    app.init_resource::<RawInputs>();
    app.init_resource::<TerminalFocus>();
    app.init_resource::<MousePosition>();
    app.add_event::<KeyPress>();
    app.add_event::<Resize>();
    app.add_systems(First, gather_input);
}

pub(crate) fn cleanup(_: &mut App) {
    std::mem::drop(stdout().execute(DisableFocusChange));
    std::mem::drop(stdout().execute(DisableMouseCapture));
    std::mem::drop(stdout().execute(DisableBracketedPaste));
}

#[derive(Default, Debug, Resource)]
pub struct RawInputs {
    raw: Vec<Event>,
}

impl RawInputs {
    fn reset(&mut self) {
        self.raw.truncate(0);
    }

    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.raw.iter()
    }
}

#[derive(Debug, Resource)]
pub struct TerminalFocus {
    pub focused: bool,
    pub focus_changed: bool
}

impl Default for TerminalFocus {
    fn default() -> Self {
        Self {
            focused: true,
            focus_changed: false,
        }
    }
}

impl TerminalFocus {
    fn reset(&mut self) {
        self.focus_changed = false;
    }

    fn set(&mut self, to: bool) {
        self.focus_changed = self.focused != to;
        self.focused = to;
    }
}

#[derive(Default, Debug, Resource)]
pub struct MousePosition(pub U16Vec2);

#[derive(bevy::ecs::event::Event, Debug)]
pub struct KeyPress {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

#[derive(bevy::ecs::event::Event, Debug, Copy, Clone)]
pub struct Resize(pub U16Vec2);

fn gather_input( 
    mut inputs: ResMut<RawInputs>,
    mut focus: ResMut<TerminalFocus>,
    mut mouse_pos: ResMut<MousePosition>,
    mut key_presses: EventWriter<KeyPress>,
    mut resize: EventWriter<Resize>,
) {
    inputs.reset();
    focus.reset();
    while poll(Duration::from_secs(0)).unwrap() {
        let event = read().unwrap();
        match event {
            Event::FocusGained => { focus.set(true); },
            Event::FocusLost => { focus.set(false); },
            Event::Mouse(event) => {
                mouse_pos.0 = U16Vec2 { x: event.column, y: event.row };
            },
            Event::Key(event) => {
                if event.kind != KeyEventKind::Release {
                    key_presses.send(KeyPress {
                        code: event.code,
                        modifiers: event.modifiers
                    });
                }
            },
            Event::Resize(x, y) => {
                resize.send(Resize(U16Vec2 { x, y, }));
            },
            _ => {},
        }
        inputs.raw.push(event);
    }
}
