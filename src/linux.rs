use std::io;

use zbus::blocking::{Connection, Proxy};

use crate::ConnectionCost;

const NM_METERED_YES: u32 = 1;
const NM_METERED_NO: u32 = 2;
const NM_METERED_GUESS_YES: u32 = 3;
const NM_METERED_GUESS_NO: u32 = 4;

pub(crate) fn connection_cost() -> io::Result<ConnectionCost> {
    match query_network_manager() {
        Ok(cost) => Ok(cost),
        Err(e) if is_service_unknown(&e) => Ok(ConnectionCost::Unknown),
        Err(e) => Err(to_io_error(e)),
    }
}

fn query_network_manager() -> zbus::Result<ConnectionCost> {
    let connection = Connection::system()?;
    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager",
        "org.freedesktop.NetworkManager",
    )?;
    let metered: u32 = proxy.get_property("Metered")?;

    Ok(match metered {
        NM_METERED_YES | NM_METERED_GUESS_YES => ConnectionCost::Metered,
        NM_METERED_NO | NM_METERED_GUESS_NO => ConnectionCost::Unmetered,
        _ => ConnectionCost::Unknown,
    })
}

fn is_service_unknown(e: &zbus::Error) -> bool {
    matches!(e, zbus::Error::MethodError(name, ..) if name.as_str() == "org.freedesktop.DBus.Error.ServiceUnknown")
}

fn to_io_error(e: zbus::Error) -> io::Error {
    let zbus::Error::InputOutput(e) = e else {
        return io::Error::other(e);
    };
    match e.raw_os_error() {
        Some(code) => io::Error::from_raw_os_error(code),
        None => io::Error::new(e.kind(), e.to_string()),
    }
}
