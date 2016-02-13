pub trait AsId<Id> {
    type IdThunk: IdThunk<Id>;
    fn into_id_thunk(self) -> Self::IdThunk;
}

pub trait IdThunk<Id> {
    fn as_id(&self) -> Id;
}

pub trait AsRaw {
    type Output;
    fn as_raw(&self) -> Self::Output;
}
