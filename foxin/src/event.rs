use bevy::{
    app::App,
    ecs::{
        schedule::ScheduleLabel,
        system::{Resource, Res, ResMut, SystemParam, Local},
    },
    utils::intern::Interned,
};
use std::{
    collections::{HashMap, VecDeque},
    marker::PhantomData,
    ops::DerefMut,
};

pub use bevy::ecs::event::{Event};

pub(crate) fn build(_: &mut App) {
}

pub(crate) fn cleanup(_: &mut App) {
}

#[derive(Debug)]
pub struct EventId<E: Event> {
    pub id: usize,
    _marker: PhantomData<E>,
}

impl<E: Event> Clone for EventId<E> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _marker: Default::default(),
        }
    }
}

impl<E: Event> Default for EventId<E> {
    fn default() -> Self {
        Self {
            id: 0,
            _marker: Default::default(),
        }
    }
}

impl<E: Event> Copy for EventId<E> {}

impl<E: Event> PartialOrd for EventId<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<E: Event> Ord for EventId<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<E: Event> PartialEq for EventId<E> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<E: Event> Eq for EventId<E> {}

#[derive(Debug)]
pub struct EventData<E: Event> {
    pub id: EventId<E>,
    pub event: E,
}

#[derive(Default, Debug, Resource)]
pub struct Events<E: Event> {
    next_id: EventId<E>,
    events: VecDeque<EventData<E>>,
    indicies: HashMap<Interned<dyn ScheduleLabel>, IndexSet<E>>, 
}

impl<E: Event> Events<E> {
    pub fn track_schedule<L: ScheduleLabel>(&mut self, label: L) {
        self.indicies.insert(label.intern(), IndexSet {
            last: self.next_id,
            before: self.next_id,
        });
    }

    pub fn advance_schedule<L: ScheduleLabel>(&mut self, label: L) {
        if let Some(set) = self.indicies.get_mut(&label.intern()) {
            set.advance(self.next_id);
        }
    }

    pub fn forget_schedule<L: ScheduleLabel>(&mut self, label: L) {
        self.indicies.remove(&label.intern());
    }

    pub fn advance(&mut self) {
        let drop_before = self
            .indicies
            .values()
            .map(|v| v.before)
            .min()
            .unwrap_or(self.next_id);

        while self.events.front().filter(|data| data.id < drop_before).is_some() {
            self.events.pop_front();
        }
    }

    pub fn send(&mut self, event: E) -> EventId<E> {
        let id = self.next_id;
        self.next_id.id += 1;
        self.events.push_back(EventData { id, event });
        id
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct IndexSet<E: Event> {
    last: EventId<E>, 
    before: EventId<E>,
}

impl<E: Event> IndexSet<E> {
    fn advance(&mut self, cur_head: EventId<E>) {
        self.before = self.last;
        self.last = cur_head;
    }
}

#[derive(Debug, SystemParam)]
pub struct EventWriter<'w, E: Event> {
    events: ResMut<'w, Events<E>>,
}

impl<'w, E: Event> EventWriter<'w, E> {
    pub fn send(&mut self, event: E) -> EventId<E> {
        self.events.send(event)
    }

    pub fn send_default(&mut self) -> EventId<E>
    where
        E: Default,
    {
        self.events.send(Default::default())
    }
}

#[derive(Debug, SystemParam)]
pub struct EventReader<'w, 's, E: Event> {
    events: Res<'w, Events<E>>,
    last_read_id: Local<'s, Option<EventId<E>>>,
}

impl<'w, 's, E: Event> EventReader<'w, 's, E> {
    pub fn read<'a>(&'a mut self) -> EventIterator<'w, 'a, E>
    where
        's: 'w,
        'a: 's,
        'a: 'w,
    {
        EventIterator {
            events: self.events.as_ref(),
            last_read_id: self.last_read_id.deref_mut(),
            cur_index: 0,
        }
    }
}

pub struct EventIterator<'w, 's, E: Event> {
    events: &'w Events<E>,
    last_read_id: &'s mut Option<EventId<E>>,
    cur_index: usize,
}

impl<'w, 's, E: Event> Iterator for EventIterator<'w, 's, E> {
    type Item = &'w EventData<E>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(evt) = self.events.events.get(self.cur_index) {
            self.cur_index += 1;

            if self.last_read_id.is_none() || matches!(self.last_read_id, Some(id) if *id < evt.id) {
                *self.last_read_id = Some(evt.id);
                return Some(evt);
            }
        }

        None
    }
}
