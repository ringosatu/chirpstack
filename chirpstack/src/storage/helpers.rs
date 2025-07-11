use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use super::schema::{application, device, device_profile, tenant};
use super::{
    application::Application, device::Device, device_profile::DeviceProfile, tenant::Tenant,
};
use super::{error::Error, get_async_db_conn};
use lrwn::EUI64;

pub async fn get_all_device_data(
    dev_eui: EUI64,
) -> Result<(Device, Application, Tenant, DeviceProfile), Error> {
    let res = device::table
        .inner_join(application::table)
        .inner_join(tenant::table.on(application::dsl::tenant_id.eq(tenant::dsl::id)))
        .inner_join(device_profile::table)
        .filter(device::dsl::dev_eui.eq(&dev_eui))
        .first::<(Device, Application, Tenant, DeviceProfile)>(&mut get_async_db_conn().await?)
        .await
        .map_err(|e| Error::from_diesel(e, dev_eui.to_string()))?;
    Ok(res)
}
