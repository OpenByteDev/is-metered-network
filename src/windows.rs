use std::{io, mem::MaybeUninit, ptr};

use windows::Win32::{
    Foundation::RPC_E_CHANGED_MODE,
    Networking::NetworkListManager::{
        INetworkCostManager, NLM_CONNECTION_COST_FIXED, NLM_CONNECTION_COST_UNRESTRICTED,
        NLM_CONNECTION_COST_VARIABLE, NetworkListManager,
    },
    System::Com::{CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx},
};

use crate::ConnectionCost;

const UNRESTRICTED_COST_MASK: u32 = NLM_CONNECTION_COST_UNRESTRICTED.0 as u32;
const METERED_COST_MASK: u32 =
    NLM_CONNECTION_COST_FIXED.0 as u32 | NLM_CONNECTION_COST_VARIABLE.0 as u32;

pub(crate) fn connection_cost() -> io::Result<ConnectionCost> {
    ensure_com_initialized()?;

    let manager: INetworkCostManager =
        unsafe { CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL) }
            .map_err(|e| to_io_error(&e))?;

    let mut cost = MaybeUninit::<u32>::uninit();
    unsafe { manager.GetCost(cost.as_mut_ptr(), ptr::null()) }.map_err(|e| to_io_error(&e))?;
    let cost = unsafe { cost.assume_init() };

    Ok(if cost & UNRESTRICTED_COST_MASK != 0 {
        ConnectionCost::Unmetered
    } else if cost & METERED_COST_MASK != 0 {
        ConnectionCost::Metered
    } else {
        ConnectionCost::Unknown
    })
}

fn ensure_com_initialized() -> io::Result<()> {
    match unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.ok() {
        Ok(()) => Ok(()),
        Err(e) if e.code() == RPC_E_CHANGED_MODE => Ok(()),
        Err(e) => Err(to_io_error(&e)),
    }
}

fn to_io_error(e: &windows::core::Error) -> io::Error {
    io::Error::from_raw_os_error(e.code().0)
}
