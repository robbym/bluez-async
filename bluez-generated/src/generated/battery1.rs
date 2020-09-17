// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.Battery1.xml --interfaces=org.bluez.Battery1 --client=nonblock --methodtype=none`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezBattery1 {
    fn percentage(&self) -> nonblock::MethodReply<u8>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> OrgBluezBattery1 for nonblock::Proxy<'a, C> {

    fn percentage(&self) -> nonblock::MethodReply<u8> {
        <Self as nonblock::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Battery1", "Percentage")
    }
}
