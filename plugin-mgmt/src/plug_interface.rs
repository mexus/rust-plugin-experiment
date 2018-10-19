use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// A plugin interface that holds a *phantom* reference to the dynamic library it has been loaded
/// from.
pub struct PlugInterface<'lib, T: ?Sized> {
    plugin_interface: Box<T>,
    lib_reference: PhantomData<&'lib ()>,
}

impl<'lib, T: ?Sized> PlugInterface<'lib, T> {
    pub fn new(plugin_interface: Box<T>) -> Self {
        PlugInterface {
            plugin_interface,
            lib_reference: PhantomData,
        }
    }
}

impl<'lib, T: ?Sized> Deref for PlugInterface<'lib, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.plugin_interface
    }
}

impl<'lib, T: ?Sized> DerefMut for PlugInterface<'lib, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.plugin_interface
    }
}
