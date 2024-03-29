use std::os::unix::prelude::AsRawFd;
use std::sync::mpsc;
use std::thread;
use std::{convert::TryInto, time::Duration};

use libc::{poll, pollfd};
use zbus::{dbus_interface, fdo};

mod libinput;
struct Greeter {}

#[dbus_interface(name = "org.gestureImprovements.gestures")]
impl Greeter {
    #[dbus_interface(signal)]
    fn touchpad_swipe(&self, event: &libinput::CustomSwipeEvent) -> zbus::Result<()>;

    #[dbus_interface(signal)]
    fn touchpad_hold(&self, event: &libinput::CustomHoldEvent) -> zbus::Result<()>;

    #[dbus_interface(signal)]
    fn touchpad_pinch(&self, event: &libinput::CustomPinchEvent) -> zbus::Result<()>;

    fn get_version(&mut self) -> String {
        const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
        VERSION.unwrap_or("unknown").into()
    }
}

fn display_info(arguments: Vec<String>) {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const COMMIT: Option<&'static str> = option_env!("GIT_HEAD_SHA");

    match arguments[1].as_str() {
        "--version" => {
            println!("version: {}", VERSION.unwrap_or("unknown"));
            if let Some(commit) = COMMIT {
                println!("commit: {}", commit)
            }
        }

        _ => {
            println!(
                "Unknown argument: {:}\n\
                Supported arguments:\n\
                 \t--version\tdisplay version information\n\
                \nRun without arguments to start dbus service",
                &arguments[1 .. arguments.len()].join(" ")
            );
        }
    }
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() > 1 {
        display_info(arguments);
        return;
    }

    let (transmitter, reciever) = mpsc::channel();

    let connection = zbus::Connection::new_session().unwrap();

    fdo::DBusProxy::new(&connection)
        .unwrap()
        .request_name(
            "org.gestureImprovements.gestures",
            fdo::RequestNameFlags::ReplaceExisting.into(),
        )
        .unwrap();

    let mut object_server = zbus::ObjectServer::new(&connection);
    let greeter = Greeter {};
    let path = &"/org/gestureImprovements/gestures".try_into().unwrap();
    object_server.at(path, greeter).unwrap();

    thread::spawn(|| {
        libinput::libinput_listener(transmitter);
    });

    // println!("starting loop");

    let timeout = Duration::from_millis(1000);

    const POLLIN: i16 = 1; // available to read

    let mut poll_fds = pollfd {
        fd: connection.as_raw_fd(),
        events: POLLIN,
        revents: 0,
    };

    let starve_limit = 16;
    let mut msg_recv = starve_limit;

    loop {
        let msg = reciever.recv_timeout(timeout);

        match msg {
            Ok(msg) => {
                object_server
                    .with(path, move |iface: &Greeter| match &msg {
                        libinput::CustomGestureEvent::Hold(hold) => iface.touchpad_hold(hold),
                        libinput::CustomGestureEvent::Swipe(swipe) => iface.touchpad_swipe(swipe),
                        libinput::CustomGestureEvent::Pinch(pinch) => iface.touchpad_pinch(pinch),
                    })
                    .unwrap();
                msg_recv += 1;
            }

            Err(_) => {
                msg_recv = starve_limit;
            }
        }

        // avoid unnessary polling, when msg was recieved from channel
        // but don't starve requests, when msgs are available continously
        if msg_recv >= starve_limit {
            msg_recv = 0;
            unsafe {
                let cc = poll(&mut poll_fds, 1, 0);
                if cc > 0 {
                    object_server.try_handle_next().unwrap();
                }
            }
        }
    }
}
