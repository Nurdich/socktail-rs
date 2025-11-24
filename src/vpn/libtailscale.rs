//! FFI bindings to libtailscale C library

use std::os::raw::{c_char, c_int};

/// Handle to a Tailscale server
pub type Tailscale = c_int;

/// Connection to a tailnet address
pub type TailscaleConn = c_int;

/// Socket listening for tailnet connections
pub type TailscaleListener = c_int;

#[link(name = "tailscale")]
extern "C" {
    /// Creates a tailscale server object
    pub fn tailscale_new() -> Tailscale;

    /// Connects server to tailnet
    pub fn tailscale_start(sd: Tailscale) -> c_int;

    /// Connects and waits for usability
    pub fn tailscale_up(sd: Tailscale) -> c_int;

    /// Shuts down the server
    pub fn tailscale_close(sd: Tailscale) -> c_int;

    /// Set state directory
    pub fn tailscale_set_dir(sd: Tailscale, dir: *const c_char) -> c_int;

    /// Set hostname
    pub fn tailscale_set_hostname(sd: Tailscale, hostname: *const c_char) -> c_int;

    /// Set auth key
    pub fn tailscale_set_authkey(sd: Tailscale, authkey: *const c_char) -> c_int;

    /// Set control URL
    pub fn tailscale_set_control_url(sd: Tailscale, control_url: *const c_char) -> c_int;

    /// Set ephemeral mode
    pub fn tailscale_set_ephemeral(sd: Tailscale, ephemeral: c_int) -> c_int;

    /// Set log file descriptor
    pub fn tailscale_set_logfd(sd: Tailscale, fd: c_int) -> c_int;

    /// Dial a connection to a tailnet address
    pub fn tailscale_dial(
        sd: Tailscale,
        network: *const c_char,
        addr: *const c_char,
        conn_out: *mut TailscaleConn,
    ) -> c_int;

    /// Listen on a tailnet address
    pub fn tailscale_listen(
        sd: Tailscale,
        network: *const c_char,
        addr: *const c_char,
        listener_out: *mut TailscaleListener,
    ) -> c_int;

    /// Accept a connection
    pub fn tailscale_accept(listener: TailscaleListener, conn_out: *mut TailscaleConn) -> c_int;

    /// Get tailscale IPs
    pub fn tailscale_getips(sd: Tailscale, buf: *mut c_char, buflen: usize) -> c_int;

    /// Get remote address of a connection
    pub fn tailscale_getremoteaddr(
        l: TailscaleListener,
        conn: TailscaleConn,
        buf: *mut c_char,
        buflen: usize,
    ) -> c_int;

    /// Get loopback address
    pub fn tailscale_loopback(
        sd: Tailscale,
        addr_out: *mut c_char,
        addrlen: usize,
        proxy_cred_out: *mut c_char,
        local_api_cred_out: *mut c_char,
    ) -> c_int;

    /// Enable funnel to localhost
    pub fn tailscale_enable_funnel_to_localhost_plaintext_http1(
        sd: Tailscale,
        localhost_port: c_int,
    ) -> c_int;

    /// Get error message
    pub fn tailscale_errmsg(sd: Tailscale, buf: *mut c_char, buflen: usize) -> c_int;
}
