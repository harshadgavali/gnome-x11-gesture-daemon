use std::{
    fs::{File, OpenOptions},
    os::unix::prelude::{AsRawFd, FromRawFd, IntoRawFd, OpenOptionsExt, RawFd},
    path::Path,
    sync::{mpsc},
};

use input::{
    event::gesture::GestureSwipeEvent,
    event::Event,
    event::GestureEvent,
    ffi::{
        libinput_event_gesture_get_dx_unaccelerated, libinput_event_gesture_get_dy_unaccelerated,
        libinput_event_gesture_get_finger_count, libinput_event_gesture_get_time,
    },
    Libinput, LibinputInterface,
};
use libc::{poll, pollfd};
use serde::{Deserialize, Serialize};
use zvariant::derive::Type;

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct CustonSwipeEvent {
    pub stage: String,
    pub fingers: i32,
    pub dx: f64,
    pub dy: f64,
    pub time: u32,
}

struct Interface;

impl LibinputInterface for Interface {
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

pub fn libinput_listener(transmitter: mpsc::Sender<CustonSwipeEvent>) {
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

            if let Event::Gesture(GestureEvent::Swipe(swipe)) = event.unwrap() {
                let stage = match &swipe {
                    GestureSwipeEvent::Begin(_) => "Begin",
                    GestureSwipeEvent::Update(_) => "Update",
                    GestureSwipeEvent::End(_) => "End",
                };

                let fingers;
                let dx;
                let dy;
                let time;

                unsafe {
                    let zz = input::AsRaw::as_raw_mut(&swipe);
                    fingers = libinput_event_gesture_get_finger_count(zz);
                    dx = libinput_event_gesture_get_dx_unaccelerated(zz);
                    dy = libinput_event_gesture_get_dy_unaccelerated(zz);
                    time = libinput_event_gesture_get_time(zz);
                }
                {
                    // println!("fingers: {}, dx: {}, dy: {}", fingers, dx, dy);

                    transmitter
                        .send(CustonSwipeEvent {
                            stage: stage.into(),
                            fingers,
                            dx,
                            dy,
                            time,
                        })
                        .unwrap();
                }
            }
        }
    }
}
