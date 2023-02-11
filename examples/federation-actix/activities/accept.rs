use crate::{activities::follow::Follow, instance::DatabaseHandle, objects::person::MyUser};
use activitypub_federation::{
    core::object_id::ObjectId,
    request_data::RequestData,
    traits::ActivityHandler,
};
use activitystreams_kinds::activity::AcceptType;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Accept {
    actor: ObjectId<MyUser>,
    object: Follow,
    #[serde(rename = "type")]
    kind: AcceptType,
    id: Url,
}

impl Accept {
    pub fn new(actor: ObjectId<MyUser>, object: Follow, id: Url) -> Accept {
        Accept {
            actor,
            object,
            kind: Default::default(),
            id,
        }
    }
}

#[async_trait::async_trait]
impl ActivityHandler for Accept {
    type DataType = DatabaseHandle;
    type Error = crate::error::Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn receive(self, _data: &RequestData<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }
}
