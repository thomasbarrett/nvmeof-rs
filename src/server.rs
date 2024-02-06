use nvmet_rs::nvme;
use nvmet_rs::nvme_tcp;
use nvmet_rs::nvme_tcp::nvme_completion_nvme_result;
use nvmet_rs::nvme_tcp::nvmf_connect_command;
use std::default;
use std::io::prelude::*;
use std::io::IoSlice;
use std::net::{TcpStream, TcpListener};

unsafe fn as_u8_slice<'a, T: Sized>(p: &'a T) -> &'a [u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

unsafe fn as_u8_slice_mut<'a, T: Sized>(p: &'a mut T) -> &'a mut [u8] {
    ::core::slice::from_raw_parts_mut(
        (p as *mut T) as *mut u8,
        ::core::mem::size_of::<T>(),
    )
}

fn main() {
    let listener: TcpListener = TcpListener::bind("172.27.22.158:4420").unwrap();
    let (mut stream, _) = listener.accept().unwrap();
    
    let mut icreq_pdu = nvme_tcp::nvme_tcp_icreq_pdu::default();
    stream.read(unsafe{ as_u8_slice_mut(&mut icreq_pdu) }).unwrap();

    let icresp_pdu = nvme_tcp::nvme_tcp_icresp_pdu{
        hdr: nvme_tcp::nvme_tcp_hdr {
            type_: nvme_tcp::nvme_tcp_pdu_type_nvme_tcp_icresp as u8,
            hlen: std::mem::size_of::<nvme_tcp::nvme_tcp_icresp_pdu>() as u8,
            plen: std::mem::size_of::<nvme_tcp::nvme_tcp_icresp_pdu>() as u32,
            ..Default::default()
        },
        maxdata: 0x400000,
        ..Default::default()
    };
    stream.write(unsafe{ as_u8_slice(&icresp_pdu) }).unwrap();

    let mut cmd_pdu = nvme_tcp::nvme_tcp_cmd_pdu::default();
    stream.read(unsafe{ as_u8_slice_mut(&mut cmd_pdu) }).unwrap();

    let mut cmd_data = nvme::nvmf_connect_data::default();
    stream.read(unsafe{ as_u8_slice_mut(&mut cmd_data) }).unwrap();

    let resp_pdu = nvme_tcp::nvme_tcp_rsp_pdu {
        hdr: nvme_tcp::nvme_tcp_hdr {
            type_: nvme_tcp::nvme_tcp_pdu_type_nvme_tcp_rsp as u8,
            hlen: std::mem::size_of::<nvme_tcp::nvme_tcp_rsp_pdu>() as u8,
            plen: std::mem::size_of::<nvme_tcp::nvme_tcp_rsp_pdu>() as u32,
            ..Default::default()
        },
        cqe: nvme_tcp::nvme_completion {
            result: nvme_tcp::nvme_completion_nvme_result{
                u64_: 1
            },
            sq_head: 1,
            sq_id: 0,
            command_id: unsafe{ cmd_pdu.cmd.__bindgen_anon_1.common.command_id },
            status: 0x00,
        }
    };

    stream.write(unsafe{ as_u8_slice(&resp_pdu) }).unwrap();

    let mut pdu_hdr = nvme_tcp::nvme_tcp_hdr::default();
    stream.read(unsafe{ as_u8_slice_mut(&mut pdu_hdr) }).unwrap();
    println!("{:?}", pdu_hdr);
}
