//! Module: zmq

#[link(name = "zmq",
       vers = "0.4",
       uuid = "54cc1bc9-02b8-447c-a227-75ebc923bc29")];
#[crate_type = "lib"];

extern mod extra;

use std::cast;
use std::libc::{c_int, c_long, c_void, size_t, c_char};
use std::ptr;
use std::str;
use std::sys;
use std::vec;

/// The ZMQ container that manages all the sockets
type Context_ = *c_void;

/// A ZMQ socket
type Socket_ = *c_void;

/// A message
type Msg_ = [c_char, ..32];

#[link_args = "-lzmq"]
extern {
    fn zmq_version(major: *c_int, minor: *c_int, patch: *c_int);

    fn zmq_init(io_threads: c_int) -> Context_;
    fn zmq_term(ctx: Context_) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> *c_char;

    fn zmq_socket(ctx: Context_, typ: c_int) -> Socket_;
    fn zmq_close(socket: Socket_) -> c_int;

    fn zmq_getsockopt(
            socket: Socket_,
            opt: c_int,
            optval: *c_void,
            size: *size_t) -> c_int;
    fn zmq_setsockopt(
            socket: Socket_,
            opt: c_int,
            optval: *c_void,
            size: size_t) -> c_int;

    fn zmq_bind(socket: Socket_, endpoint: *c_char) -> c_int;
    fn zmq_connect(socket: Socket_, endpoint: *c_char) -> c_int;

    fn zmq_msg_init(msg: &Msg_) -> c_int;
    fn zmq_msg_init_size(msg: &Msg_, size: size_t) -> c_int;
    fn zmq_msg_data(msg: &Msg_) -> *u8;
    fn zmq_msg_size(msg: &Msg_) -> size_t;
    fn zmq_msg_close(msg: &Msg_) -> c_int;

    fn zmq_msg_send(msg: &Msg_, socket: Socket_, flags: c_int) -> c_int;
    fn zmq_msg_recv(msg: &Msg_, socket: Socket_, flags: c_int) -> c_int;

    fn zmq_poll(items: *PollItem, nitems: c_int, timeout: c_long) -> c_int;
}

/// Socket types
#[deriving(Clone)]
pub enum SocketType {
    PAIR   = 0,
    PUB    = 1,
    SUB    = 2,
    REQ    = 3,
    REP    = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL   = 7,
    PUSH   = 8,
    XPUB   = 9,
    XSUB   = 10,
}

pub static DONTWAIT : int = 1;
pub static SNDMORE : int = 2;

#[deriving(Clone)]
pub enum Constants {
    ZMQ_AFFINITY          = 4,
    ZMQ_IDENTITY          = 5,
    ZMQ_SUBSCRIBE         = 6,
    ZMQ_UNSUBSCRIBE       = 7,
    ZMQ_RATE              = 8,
    ZMQ_RECOVERY_IVL      = 9,
    ZMQ_MCAST_LOOP        = 10,
    ZMQ_SNDBUF            = 11,
    ZMQ_RCVBUF            = 12,
    ZMQ_RCVMORE           = 13,
    ZMQ_FD                = 14,
    ZMQ_EVENTS            = 15,
    ZMQ_TYPE              = 16,
    ZMQ_LINGER            = 17,
    ZMQ_RECONNECT_IVL     = 18,
    ZMQ_BACKLOG           = 19,
    ZMQ_RECOVERY_IVL_MSEC = 20,
    ZMQ_RECONNECT_IVL_MAX = 21,
    ZMQ_MAXMSGSIZE        = 22,
    ZMQ_SNDHWM            = 23,
    ZMQ_RCVHWM            = 24,

    ZMQ_MAX_VSM_SIZE      = 30,
    ZMQ_DELIMITER         = 31,
    ZMQ_VSM               = 32,

    ZMQ_MSG_MORE          = 1,
    ZMQ_MSG_SHARED        = 128,
    ZMQ_MSG_MASK          = 129,

    ZMQ_HAUSNUMERO        = 156384712,
}

impl Constants {
    pub fn to_raw(&self) -> i32 {
        *self as i32
    }

    pub fn from_raw(raw: i32) -> Constants {
        // fails if `raw` is not a valid value
        match raw {
            4         => ZMQ_AFFINITY,
            5         => ZMQ_IDENTITY,
            6         => ZMQ_SUBSCRIBE,
            7         => ZMQ_UNSUBSCRIBE,
            8         => ZMQ_RATE,
            9         => ZMQ_RECOVERY_IVL,
            10        => ZMQ_MCAST_LOOP,
            11        => ZMQ_SNDBUF,
            12        => ZMQ_RCVBUF,
            13        => ZMQ_RCVMORE,
            14        => ZMQ_FD,
            15        => ZMQ_EVENTS,
            16        => ZMQ_TYPE,
            17        => ZMQ_LINGER,
            18        => ZMQ_RECONNECT_IVL,
            19        => ZMQ_BACKLOG,
            20        => ZMQ_RECOVERY_IVL_MSEC,
            21        => ZMQ_RECONNECT_IVL_MAX,
            22        => ZMQ_MAXMSGSIZE,
            23        => ZMQ_SNDHWM,
            24        => ZMQ_RCVHWM,

            30        => ZMQ_MAX_VSM_SIZE,
            31        => ZMQ_DELIMITER,
            32        => ZMQ_VSM,

            1         => ZMQ_MSG_MORE,
            128       => ZMQ_MSG_SHARED,
            129       => ZMQ_MSG_MASK,

            156384712 => ZMQ_HAUSNUMERO,

            x         => fail!("invalid constant %d", x as int),
        }
    }
}

#[deriving(Clone)]
pub enum Error {
    // the magic number is ZMQ_HAUSNUMERO
    ENOTSUP         = 156384712 + 1,
    EPROTONOSUPPORT = 156384712 + 2,
    ENOBUFS         = 156384712 + 3,
    ENETDOWN        = 156384712 + 4,
    EADDRINUSE      = 156384712 + 5,
    EADDRNOTAVAIL   = 156384712 + 6,
    ECONNREFUSED    = 156384712 + 7,
    EINPROGRESS     = 156384712 + 8,
    ENOTSOCK        = 156384712 + 9,

    EFSM            = 156384712 + 51,
    ENOCOMPATPROTO  = 156384712 + 52,
    ETERM           = 156384712 + 53,
    EMTHREAD        = 156384712 + 54,
}

// Return the current zeromq version.
pub fn version() -> (int, int, int) {
    let major = 0i32;
    let minor = 0i32;
    let patch = 0i32;
    unsafe {
        zmq_version(
            &major,
            &minor,
            &patch);
    }
    (major as int, minor as int, patch as int)
}

// Create a zeromq context.
pub fn init(io_threads: int) -> Result<Context, Error> {    
    let ctx = unsafe {zmq_init(io_threads as i32)};

    if ctx.is_null() {
        return Err(errno_to_error());
    }

    Ok(Context { ctx: ctx })
}

/// zmq context, used to create sockets. Is thread safe, and can be safely
/// shared, but dropping it while sockets are still open will cause them to
/// close (see zmq_ctx_destroy(3)).
///
/// For this reason, one should use an Arc to share it, rather than any unsafe
/// trickery you might think up that would call the destructor.
pub struct Context {
    priv ctx: Context_,
}

impl Context {
    pub fn socket(&self, socket_type: SocketType) -> Result<Socket, Error> {
        let sock = unsafe {zmq_socket(self.ctx, socket_type as c_int)};

        if sock.is_null() {
            return Err(errno_to_error());
        }

        Ok(Socket { sock: sock as Socket_, closed: false })
    }

    pub fn term(&self) -> Result<(), Error> {
        if unsafe { zmq_term(self.ctx) } == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(())
        }
    }
}

pub struct Socket {
    priv sock: Socket_,
    priv closed: bool
}

impl Drop for Socket {
    pub fn drop(&self) {
        match self.close_final() {
            Ok(()) => {},
            Err(e) => fail!(e.to_str())
        }
    }
}

impl Socket {
    pub fn get_socket_type(&self) -> Result<SocketType, Error> {
        do getsockopt_int(self.sock, ZMQ_TYPE.to_raw()).map |ty| {
            match *ty {
                0 => PAIR,
                1 => PUB,
                2 => SUB,
                3 => REQ,
                4 => REP,
                5 => DEALER,
                6 => ROUTER,
                7 => PULL,
                8 => PUSH,
                9 => XPUB,
                10 => XSUB,
                _ => fail!(~"socket type is out of range!")
            }
        }
    }

    pub fn get_rcvmore(&self) -> Result<bool, Error> {
        do getsockopt_i64(self.sock, ZMQ_RCVMORE.to_raw()).chain |o| {
            Ok(o == 1i64)
        }
    }

    pub fn get_maxmsgsize(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_MAXMSGSIZE.to_raw())
    }


    pub fn get_sndhwm(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_SNDHWM.to_raw())
    }

    pub fn get_rcvhwm(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_RCVHWM.to_raw())
    }

    pub fn get_affinity(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, ZMQ_AFFINITY.to_raw())
    }

    pub fn get_identity(&self) -> Result<~[u8], Error> {
        getsockopt_bytes(self.sock, ZMQ_IDENTITY.to_raw())
    }

    pub fn get_rate(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_RATE.to_raw())
    }

    pub fn get_recovery_ivl(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_RECOVERY_IVL.to_raw())
    }

    pub fn get_recovery_ivl_msec(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_RECOVERY_IVL_MSEC.to_raw())
    }

    pub fn get_mcast_loop(&self) -> Result<bool, Error> {
        do getsockopt_i64(self.sock, ZMQ_MCAST_LOOP.to_raw()).chain |o| {
            Ok(o == 1i64)
        }
    }

    pub fn get_sndbuf(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, ZMQ_SNDBUF.to_raw())
    }

    pub fn get_rcvbuf(&self) -> Result<u64, Error> {
        getsockopt_u64(self.sock, ZMQ_RCVBUF.to_raw())
    }

    pub fn get_linger(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_LINGER.to_raw())
    }

    pub fn get_reconnect_ivl(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_RECONNECT_IVL.to_raw())
    }

    pub fn get_reconnect_ivl_max(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_RECONNECT_IVL_MAX.to_raw())
    }

    pub fn get_backlog(&self) -> Result<int, Error> {
        getsockopt_int(self.sock, ZMQ_BACKLOG.to_raw())
    }

    pub fn get_fd(&self) -> Result<i64, Error> {
        getsockopt_i64(self.sock, ZMQ_FD.to_raw())
    }

    pub fn get_events(&self) -> Result<u32, Error> {
        getsockopt_u32(self.sock, ZMQ_EVENTS.to_raw())
    }

    pub fn set_maxmsgsize(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_MAXMSGSIZE.to_raw(), value)
    }


    pub fn set_sndhwm(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_SNDHWM.to_raw(), value)
    }

    pub fn set_rcvhwm(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_RCVHWM.to_raw(), value)
    }

    pub fn set_affinity(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, ZMQ_AFFINITY.to_raw(), value)
    }

    pub fn set_identity(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, ZMQ_IDENTITY.to_raw(), value)
    }

    pub fn set_subscribe(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, ZMQ_SUBSCRIBE.to_raw(), value)
    }

    pub fn set_unsubscribe(&self, value: &[u8]) -> Result<(), Error> {
        setsockopt_bytes(self.sock, ZMQ_UNSUBSCRIBE.to_raw(), value)
    }

    pub fn set_rate(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_RATE.to_raw(), value)
    }

    pub fn set_recovery_ivl(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_RECOVERY_IVL.to_raw(), value)
    }

    pub fn set_recovery_ivl_msec(&self, value: i64) -> Result<(), Error> {
        setsockopt_i64(self.sock, ZMQ_RECOVERY_IVL_MSEC.to_raw(), value)
    }

    pub fn set_mcast_loop(&self, value: bool) -> Result<(), Error> {
        let value = if value { 1i64 } else { 0i64 };
        setsockopt_i64(self.sock, ZMQ_MCAST_LOOP.to_raw(), value)
    }

    pub fn set_sndbuf(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, ZMQ_SNDBUF.to_raw(), value)
    }

    pub fn set_rcvbuf(&self, value: u64) -> Result<(), Error> {
        setsockopt_u64(self.sock, ZMQ_RCVBUF.to_raw(), value)
    }

    pub fn set_linger(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_LINGER.to_raw(), value)
    }

    pub fn set_reconnect_ivl(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_RECONNECT_IVL.to_raw(), value)
    }

    pub fn set_reconnect_ivl_max(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_RECONNECT_IVL_MAX.to_raw(), value)
    }

    pub fn set_backlog(&self, value: int) -> Result<(), Error> {
        setsockopt_int(self.sock, ZMQ_BACKLOG.to_raw(), value)
    }

    /// Accept connections on a socket.
    pub fn bind(&self, endpoint: &str) -> Result<(), Error> {
        let rc = do endpoint.as_c_str |cstr| {
            unsafe {zmq_bind(self.sock, cstr)}
        };

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    /// Connect a socket.
    pub fn connect(&self, endpoint: &str) -> Result<(), Error> {
        let rc = do endpoint.as_c_str |cstr| {
            unsafe {zmq_connect(self.sock, cstr)}
        };

        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }

    pub fn send(&self, data: &[u8], flags: int) -> Result<(), Error> {
        do data.as_imm_buf |base_ptr, len| {
            let msg = [0, ..32];

            unsafe {
                // Copy the data into the message.
                zmq_msg_init_size(&msg, len as size_t);

                ptr::copy_memory(
                    ::cast::transmute(zmq_msg_data(&msg)),
                    base_ptr,
                    len);

                let rc = zmq_msg_send(&msg, self.sock, flags as c_int);

                zmq_msg_close(&msg);

                if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
            }
        }
    }

    pub fn send_str(&self, data: &str, flags: int) -> Result<(), Error> {
        self.send(data.as_bytes(), flags)
    }

    pub unsafe fn recv(&self, flags: int) -> Result<Message, Error> {
        let msg = [0, ..32];

        zmq_msg_init(&msg);
        let rc = zmq_msg_recv(&msg, self.sock, flags as c_int);

        if rc == -1i32 {
            Err(errno_to_error())
        } else {
            Ok(Message { msg: msg })
        }
    }

    pub fn recv_bytes(&self, flags: int) -> Result<~[u8], Error> {
        match unsafe { self.recv(flags) } {
            Ok(msg) => Ok(msg.to_bytes()),
            Err(e) => Err(e),
        }
    }

    pub fn recv_str(&self, flags: int) -> Result<~str, Error> {
        match unsafe { self.recv(flags) } {
            Ok(msg) => Ok(msg.to_str()),
            Err(e) => Err(e),
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if !self.closed {
            self.closed = true;

            if unsafe { zmq_close(self.sock) } == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }

    pub fn close_final(&self) -> Result<(), Error> {
        if !self.closed {
            if unsafe { zmq_close(self.sock) } == -1i32 {
                return Err(errno_to_error());
            }
        }

        Ok(())
    }
}

struct Message {
    priv msg: Msg_
}

impl Drop for Message {
    pub fn drop(&self) {
        unsafe { zmq_msg_close(&self.msg); }
    }
}

impl Message {
    pub fn with_ptr<T>(&self, f: &fn(*u8, uint) -> T) -> T {
        unsafe {
            let data = zmq_msg_data(&self.msg);
            let len = zmq_msg_size(&self.msg) as uint;

            f(data, len)
        }
    }

    pub fn with_bytes<T>(&self, f: &fn(&[u8]) -> T) -> T {
        do self.with_ptr |data, len| {
            unsafe { vec::raw::buf_as_slice(data, len, |x| f(x)) }
        }
    }

    pub fn with_str<T>(&self, f: &fn(&str) -> T) -> T {
        self.with_bytes(|v| f(str::from_bytes_slice(v)))
    }

    pub fn to_bytes(&self) -> ~[u8] {
        self.with_bytes(|v| v.to_owned())
    }

    pub fn to_str(&self) -> ~str {
        self.with_str(|s| s.to_owned())
    }
}

pub static POLLIN : i16 = 1i16;
pub static POLLOUT : i16 = 2i16;
pub static POLLERR : i16 = 4i16;

pub struct PollItem {
    socket: Socket_,
    fd: c_int,
    events: i16,
    revents: i16
}

pub fn poll(items: &[PollItem], timeout: i64) -> Result<(), Error> {
    do items.as_imm_buf |p, len| {
        let rc = unsafe {zmq_poll(
            p,
            len as c_int,
            timeout as c_long)};
        if rc == -1i32 { Err(errno_to_error()) } else { Ok(()) }
    }
}

impl ToStr for Error {
    /// Return the error string for an error.
    fn to_str(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(zmq_strerror(*self as c_int))
        }
    }
}

/// Convert the errno into an error type.
fn errno_to_error() -> Error {
    unsafe {
        match zmq_errno() {
            e if e == ENOTSUP as c_int         => ENOTSUP,
            e if e == EPROTONOSUPPORT as c_int => EPROTONOSUPPORT,
            e if e == ENOBUFS as c_int         => ENOBUFS,
            e if e == ENETDOWN as c_int        => ENETDOWN,
            e if e == EADDRINUSE as c_int      => EADDRINUSE,
            e if e == EADDRNOTAVAIL as c_int   => EADDRNOTAVAIL,
            e if e == ECONNREFUSED as c_int    => ECONNREFUSED,
            e if e == EINPROGRESS as c_int     => EINPROGRESS,
            e if e == ENOTSOCK as c_int        => ENOTSOCK,
            e if e == EFSM as c_int            => EFSM,
            e if e == ENOCOMPATPROTO as c_int  => ENOCOMPATPROTO,
            e if e == ETERM as c_int           => ETERM,
            e if e == EMTHREAD as c_int        => EMTHREAD,
            e => fail!(str::raw::from_c_str(zmq_strerror(e as c_int))),
        }
    }
}

fn getsockopt_int(sock: Socket_, opt: c_int) -> Result<int, Error> {
    let value = 0u32 as c_int;
    let size = sys::size_of::<c_int>() as size_t;

    let r = unsafe {
        zmq_getsockopt(
            sock,
            opt as c_int,
            ptr::to_unsafe_ptr(&value) as *c_void,
            &size)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value as int) }
}

fn getsockopt_u32(sock: Socket_, opt: c_int) -> Result<u32, Error> {
    let value = 0u32;
    let size = sys::size_of::<u32>() as size_t;

    let r = unsafe {
        zmq_getsockopt(
            sock,
            opt,
            ptr::to_unsafe_ptr(&value) as *c_void,
            &size)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value) }
}

fn getsockopt_i64(sock: Socket_, opt: c_int) -> Result<i64, Error> {
    let value = 0i64;
    let size = sys::size_of::<i64>() as size_t;

    let r = unsafe {
        zmq_getsockopt(
            sock,
            opt as c_int,
            ptr::to_unsafe_ptr(&value) as *c_void,
            &size)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value) }
}

fn getsockopt_u64(sock: Socket_, opt: c_int) -> Result<u64, Error> {
    let value = 0u64;
    let size = sys::size_of::<u64>() as size_t;

    let r = unsafe {
        zmq_getsockopt(
            sock,
            opt,
            ptr::to_unsafe_ptr(&value) as *c_void,
            &size)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(value) }
}

fn getsockopt_bytes(sock: Socket_, opt: c_int) -> Result<~[u8], Error> {
    // The only binary option in zeromq is ZMQ_IDENTITY, which can have
    // a max size of 255 bytes.
    let size = 255 as size_t;
    let mut value = vec::with_capacity(size as uint);

    let r = unsafe {zmq_getsockopt(
        sock,
        opt as c_int,
        vec::raw::to_ptr(value) as *c_void,
        &size)};

    if r == -1i32 {
        Err(errno_to_error())
    } else {
        unsafe {vec::raw::set_len(&mut value, size as uint)};
        Ok(value)
    }
}

fn setsockopt_int(sock: Socket_, opt: c_int, value: int) -> Result<(), Error> {
    let value = value as c_int;
    let r = unsafe {
        zmq_setsockopt(
            sock,
            opt as c_int,
            ptr::to_unsafe_ptr(&value) as *c_void,
            sys::size_of::<c_int>() as size_t)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_i64(sock: Socket_, opt: c_int, value: i64) -> Result<(), Error> {
    let r = unsafe {
        zmq_setsockopt(
            sock,
            opt as c_int,
            ptr::to_unsafe_ptr(&value) as *c_void,
            sys::size_of::<i64>() as size_t)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_u64(sock: Socket_, opt: c_int, value: u64) -> Result<(), Error> {
    let r = unsafe {
        zmq_setsockopt(
            sock,
            opt as c_int,
            ptr::to_unsafe_ptr(&value) as *c_void,
            sys::size_of::<u64>() as size_t)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_buf(sock: Socket_, opt: c_int, p: *u8, len: uint) -> Result<(), Error> {
    let r = unsafe {
        zmq_setsockopt(
            sock,
            opt as c_int,
            p as *c_void,
            len as size_t)
    };

    if r == -1i32 { Err(errno_to_error()) } else { Ok(()) }
}

fn setsockopt_bytes( sock: Socket_, opt: c_int, value: &[u8]) -> Result<(), Error> {
    value.as_imm_buf(|p, len| setsockopt_buf(sock, opt, p, len))
}

fn setsockopt_str(sock: Socket_, opt: c_int, value: &str) -> Result<(), Error> {
    value.as_bytes().as_imm_buf(|bytes, len| setsockopt_buf(sock, opt, bytes, len))
}
