use libloading::{self, Library, Result};
use plug_interface::PlugInterface;
use std::ops::{Deref, DerefMut};

rental! {
    mod rent {
        use libloading::Library;
        use plug_interface::PlugInterface;

        #[rental(deref_mut_suffix)]
        pub struct RentInterface<T: 'static + ?Sized> {
            lib: Box<Library>,
            interface: PlugInterface<'lib, T>,
        }
    }
}

pub struct SharedLibPlugin<T: ?Sized + 'static>(rent::RentInterface<T>);

impl<T: ?Sized> SharedLibPlugin<T> {
    pub(crate) fn load_plugin(lib: Library, symbol: &[u8]) -> Result<Self> {
        let lib = Box::new(lib);
        rent::RentInterface::try_new_or_drop(lib, |lib| {
            let func: libloading::Symbol<fn() -> Box<T>> = unsafe { lib.get(symbol) }?;
            let plugin: Box<T> = func();
            Ok(PlugInterface::new(plugin))
        })
        .map(SharedLibPlugin)
    }

    pub(crate) fn load_plugin_arg<Arg>(lib: Library, symbol: &[u8], argument: Arg) -> Result<Self> {
        let lib = Box::new(lib);
        rent::RentInterface::try_new_or_drop(lib, |lib| {
            let func: libloading::Symbol<fn(Arg) -> Box<T>> = unsafe { lib.get(symbol) }?;
            let plugin: Box<T> = func(argument);
            Ok(PlugInterface::new(plugin))
        })
        .map(SharedLibPlugin)
    }
}

impl<T: ?Sized> Deref for SharedLibPlugin<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0.deref()
    }
}

impl<T: ?Sized> DerefMut for SharedLibPlugin<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.deref_mut()
    }
}
