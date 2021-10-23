use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::{AsRawFd, FromRawFd, IntoRawFd, OpenOptionsExt, RawFd},
    path::Path,
    sync::mpsc,
};

use input::{
    event::gesture::GestureSwipeEvent,
    event::Event,
    event::{gesture::GestureHoldEvent, GestureEvent},
    ffi::{
        libinput_event_gesture_get_cancelled, libinput_event_gesture_get_dx_unaccelerated,
        libinput_event_gesture_get_dy_unaccelerated, libinput_event_gesture_get_finger_count,
        libinput_event_gesture_get_time,
    },
    Libinput, LibinputInterface,
};
use libc::{poll, pollfd};
use serde::{Deserialize, Serialize};
use zvariant::derive::Type;

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct CustomSwipeEvent {
    pub stage: String,
    pub fingers: i32,
    pub dx: f64,
    pub dy: f64,
    pub time: u32,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct CustomHoldEvent {
    pub stage: String,
    pub fingers: i32,
    pub time: u32,
    pub is_cancelled: bool,
}

pub enum CustomGestureEvent {
    Swipe(CustomSwipeEvent),
    Hold(CustomHoldEvent),
}

struct Interface;

impl LibinputInterface for Interface {
    #[allow(clippy::bad_bit_mask)]
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & libc::O_RDONLY != 0) | (flags & libc::O_RDWR != 0))
            .write((flags & libc::O_WRONLY != 0) | (flags & libc::O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}

pub fn handle_swipe(swipe: GestureSwipeEvent, transmitter: &mpsc::Sender<CustomGestureEvent>) {
    let stage = match &swipe {
        GestureSwipeEvent::Begin(_) => "Begin",
        GestureSwipeEvent::Update(_) => "Update",
        GestureSwipeEvent::End(_) => "End",
        _ => panic!("Unkown gesture event {:?}", swipe),
    };

    let (fingers, dx, dy, time) = unsafe {
        let raw_gesture_event = input::AsRaw::as_raw_mut(&swipe);
        (
            libinput_event_gesture_get_finger_count(raw_gesture_event),
            libinput_event_gesture_get_dx_unaccelerated(raw_gesture_event),
            libinput_event_gesture_get_dy_unaccelerated(raw_gesture_event),
            libinput_event_gesture_get_time(raw_gesture_event),
        )
    };

    let swipe = CustomSwipeEvent {
        stage: stage.into(),
        fingers,
        dx,
        dy,
        time,
    };

    transmitter.send(CustomGestureEvent::Swipe(swipe)).unwrap();
}

pub fn handle_hold(hold: GestureHoldEvent, transmitter: &mpsc::Sender<CustomGestureEvent>) {
    let stage = match &hold {
        GestureHoldEvent::Begin(_) => "Begin",
        GestureHoldEvent::End(_) => "End",
        _ => panic!("Unkown gesture event {:?}", hold),
    };

    let (fingers, time, is_cancelled) = unsafe {
        let raw_gesture_event = input::AsRaw::as_raw_mut(&hold);
        (
            libinput_event_gesture_get_finger_count(raw_gesture_event),
            libinput_event_gesture_get_time(raw_gesture_event),
            // calling libinput_event_gesture_get_cancelled on begin gesture in error
            matches!(hold, GestureHoldEvent::End(_))
                && libinput_event_gesture_get_cancelled(raw_gesture_event) != 0,
        )
    };

    // only send >= 3 finger hold gestures
    if fingers < 3 {
        return;
    }

    let hold = CustomHoldEvent {
        stage: stage.into(),
        fingers,
        time,
        is_cancelled,
    };

    transmitter.send(CustomGestureEvent::Hold(hold)).unwrap();
}

pub fn libinput_listener(transmitter: mpsc::Sender<CustomGestureEvent>) {
    let mut input = Libinput::new_with_udev(Interface);

    input.udev_assign_seat("seat0").unwrap();

    const POLLIN: i16 = 1; // available to read

    let mut poll_fds = pollfd {
        fd: input.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    };

    loop {
        unsafe {
            poll(&mut poll_fds, 1, -1);
        }

        input.dispatch().unwrap();
        loop {
            let event = input.next();
            if event.is_none() {
                break;
            }

            if let Event::Gesture(gesture_event) = event.unwrap() {
                match gesture_event {
                    GestureEvent::Hold(hold) => handle_hold(hold, &transmitter),
                    GestureEvent::Swipe(swipe) => handle_swipe(swipe, &transmitter),
                    _ => {}
                }
            }
        }
    }
}
