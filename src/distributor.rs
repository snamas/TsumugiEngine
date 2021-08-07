use crate::TsumugiAnyTrait;
use std::any::TypeId;

pub enum ParcelLifeTime {
    Shot,
    Cold,
    Lifetime(u32),
    LifeCount(u32),
    Update
}
pub struct TsumugiParcelDistributor {
    pub parcel:Box<dyn TsumugiAnyTrait + Send>,
    pub parceltype: TypeId,
    pub parcellifetime: ParcelLifeTime,
    pub parcel_name: Option<String>,
}

impl TsumugiParcelDistributor{
    pub fn new<T: 'static + TsumugiAnyTrait + Send>(tsumugi_parcel:T) ->Self{
        TsumugiParcelDistributor{
            parcel: Box::new(tsumugi_parcel),
            parceltype: TypeId::of::<T>(),
            parcellifetime: ParcelLifeTime::Shot,
            parcel_name: None
        }
    }
}