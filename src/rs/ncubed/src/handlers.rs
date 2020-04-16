use warp;

use crate::actors::NcubeActor;
use crate::errors::RouteRejection;
use crate::messages::{InsertSetting, IsBootstrapped, ShowConfig};
use crate::registry::Registry;

use crate::types::NcubeConfigRequest;

pub mod ncube_config {
    use super::*;

    pub async fn show() -> Result<impl warp::Reply, warp::Rejection> {
        let mut actor = NcubeActor::from_registry().await.unwrap();

        if let Ok(false) = actor
            .call(IsBootstrapped)
            .await
            .map_err(|_| RouteRejection::ChannelError)?
        {
            // warp::reject::not_found() returns a 405 Method Not Allowed status
            // code. To ensure a 404 Not Found I create a custom rejection
            // error (RouteRejection::NotFound).
            // See: https://github.com/seanmonstar/warp/issues/77
            return Err(warp::reject::custom(RouteRejection::NotFound));
        }

        let config = actor
            .call(ShowConfig)
            .await
            .map_err(|_| RouteRejection::ChannelError)?
            .map_err(|_| RouteRejection::DataError)?;

        Ok(warp::reply::json(&config))
    }

    pub async fn create(settings: NcubeConfigRequest) -> Result<impl warp::Reply, warp::Rejection> {
        let mut actor = NcubeActor::from_registry().await.unwrap();

        if let Ok(true) = actor
            .call(IsBootstrapped)
            .await
            .map_err(|_| RouteRejection::ChannelError)?
        {
            return Err(warp::reject());
        }

        for setting in settings {
            let _ = actor
                .call(InsertSetting::new(
                    setting.name.to_string(),
                    setting.value.to_string(),
                ))
                .await
                .map_err(|_| RouteRejection::ChannelError)?;
        }

        Ok(warp::reply())
    }

    pub async fn insert(settings: NcubeConfigRequest) -> Result<impl warp::Reply, warp::Rejection> {
        let mut actor = NcubeActor::from_registry().await.unwrap();

        if let Ok(false) = actor
            .call(IsBootstrapped)
            .await
            .map_err(|_| RouteRejection::ChannelError)?
        {
            return Err(warp::reject());
        }

        for setting in settings {
            let _ = actor
                .call(InsertSetting::new(
                    setting.name.to_string(),
                    setting.value.to_string(),
                ))
                .await
                .map_err(|_| RouteRejection::ChannelError)?;
        }

        Ok(warp::reply())
    }
}