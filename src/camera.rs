pub(crate) mod toupcam;

pub(crate) trait Camera {
    fn open(&self);
    fn close(&self);
}
