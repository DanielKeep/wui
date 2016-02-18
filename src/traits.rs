pub trait AsId<Id> {
    type IdThunk: IdThunk<Id>;
    fn into_id_thunk(self) -> Self::IdThunk;
}

pub trait IdThunk<Id> {
    fn as_id(&self) -> Id;
}

pub trait AsRaw {
    type Raw;
    fn as_raw(&self) -> Self::Raw;
}

impl<'a, T> AsRaw for &'a T where T: AsRaw {
    type Raw = <T as AsRaw>::Raw;

    fn as_raw(&self) -> Self::Raw {
        (*self).as_raw()
    }
}

pub trait FromRaw: AsRaw {
    unsafe fn from_raw(raw: Self::Raw) -> Self;
}

pub trait IntoRaw: AsRaw {
    fn into_raw(self) -> Self::Raw;
}
