// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.HealthManager1.xml --interfaces=org.bluez.HealthManager1 --client=nonblock --methodtype=none`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezHealthManager1 {
    fn create_application(&self, config: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> nonblock::MethodReply<dbus::Path<'static>>;
    fn destroy_application(&self, application: dbus::Path) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> OrgBluezHealthManager1 for nonblock::Proxy<'a, C> {

    fn create_application(&self, config: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> nonblock::MethodReply<dbus::Path<'static>> {
        self.method_call("org.bluez.HealthManager1", "CreateApplication", (config, ))
            .and_then(|r: (dbus::Path<'static>, )| Ok(r.0, ))
    }

    fn destroy_application(&self, application: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.HealthManager1", "DestroyApplication", (application, ))
    }
}
