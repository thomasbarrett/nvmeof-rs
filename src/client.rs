use nvmet_rs::nvme;
use nvmet_rs::nvme_tcp;
use nvmet_rs::nvme_tcp::nvmf_connect_command;
use std::default;
use std::io::prelude::*;
use std::io::IoSlice;
use std::net::TcpStream;

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
    let icreq_pdu = nvme_tcp::nvme_tcp_icreq_pdu{
        hdr: nvme_tcp::nvme_tcp_hdr {
            type_: nvme_tcp::nvme_tcp_pdu_type_nvme_tcp_icreq as u8,
            hlen: std::mem::size_of::<nvme_tcp::nvme_tcp_icreq_pdu>() as u8,
            plen: std::mem::size_of::<nvme_tcp::nvme_tcp_icreq_pdu>() as u32,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut stream = TcpStream::connect("172.27.22.113:4420").unwrap();
    stream.write(unsafe{ as_u8_slice(&icreq_pdu) }).unwrap();
    
    let mut icresp_pdu = nvme_tcp::nvme_tcp_icresp_pdu::default();
    stream.read(unsafe{ as_u8_slice_mut(&mut icresp_pdu) }).unwrap();

    let cmd_pdu = nvme_tcp::nvme_tcp_cmd_pdu{
        hdr: nvme_tcp::nvme_tcp_hdr { 
            type_: nvme_tcp::nvme_tcp_pdu_type_nvme_tcp_cmd as u8,
            hlen: std::mem::size_of::<nvme_tcp::nvme_tcp_cmd_pdu>() as u8,
            pdo: std::mem::size_of::<nvme_tcp::nvme_tcp_cmd_pdu>() as u8,
            plen: (std::mem::size_of::<nvme_tcp::nvme_tcp_cmd_pdu>() + std::mem::size_of::<nvme::nvmf_connect_data>()) as u32,
            ..Default::default()
        },
        cmd: nvme_tcp::nvme_command { 
            __bindgen_anon_1: nvme_tcp::nvme_command__bindgen_ty_1{ 
                connect: nvme_tcp::nvmf_connect_command {
                    opcode: nvme::nvmf_fabrics_opcode_nvme_fabrics_command as u8,
                    resv1: nvme::NVME_CMD_SGL_METABUF as u8,                
                    command_id: u16::to_le(4096),
                    fctype: nvme::nvmf_capsule_command_nvme_fabrics_type_connect as u8,
                    dptr: nvme_tcp::nvme_data_ptr{
                        sgl: nvme_tcp::nvme_sgl_desc {
                            addr: 0x00,
                            length: (std::mem::size_of::<nvme::nvmf_connect_data>()) as u32, 
                            type_: (nvme::NVME_SGL_FMT_DATA_DESC << 4 | nvme::NVME_SGL_FMT_OFFSET) as u8,
                            ..Default::default()
                        }
                    },
                    recfmt: 0,
                    qid: u16::to_le(0),    
                    sqsize: 31,
                    cattr: 0,
                    kato: 5000,     
                    ..Default::default()
                },
            }
        },
    };

    let hostnqn_str = "nqn.2014-08.org.nvmexpress:uuid:aa6251b3-835e-47b6-81dc-bc50e8d321e6";
    let mut hostnqn = [0x00i8; 256];
    hostnqn[..hostnqn_str.len()].copy_from_slice(unsafe{ &*(hostnqn_str.as_bytes() as *const [u8] as *const [i8]) });

    let subsysnqn_str = "test";
    let mut subsysnqn = [0x00i8; 256];
    subsysnqn[..subsysnqn_str.len()].copy_from_slice(unsafe{ &*(subsysnqn_str.as_bytes() as *const [u8] as *const [i8]) });

    let data = nvme::nvmf_connect_data{
        hostid: nvme::uuid_t { b: [0x08; 16] },
        cntlid: u16::to_le(0xffff),
        hostnqn,
        subsysnqn,
        ..Default::default()
    };

    stream.write(unsafe{ as_u8_slice(&cmd_pdu) }).unwrap();
    stream.write(unsafe{ as_u8_slice(&data) }).unwrap();

    let mut resp_pdu = nvme_tcp::nvme_tcp_rsp_pdu::default();
    stream.read(unsafe{ as_u8_slice_mut(&mut resp_pdu) }).unwrap();

    println!("{:?} {:x}", resp_pdu.hdr, unsafe{ resp_pdu.cqe.result.u64_ });
}
