use mongodb::{
    bson::{doc, oid::ObjectId, Document, Uuid},
    Client, Collection,
};

pub struct Trainingssession {
    id: Uuid,
    name: String,
    apparatus_id: Vec<String>,
}

impl Trainingssession {
    pub fn new(name: &String, apparatus_id: &Vec<String>) -> Trainingssession {
        Trainingssession {
            id: Uuid::new(),
            name: name.to_string(),
            apparatus_id: apparatus_id.to_vec(),
        }
    }

    pub async fn database_insert(
        self: Trainingssession,
        client: &Client,
        userid: ObjectId,
    ) -> Option<Uuid> {
        let user_collection: Collection<Document> = client.database("prod").collection("users");

        let update_result = user_collection.update_one(
            doc! {
                "_id": &userid,
            },
            doc! {
                "$push":{
                    "sessions": {
                        "_id": &self.id,
                        "name": self.name,
                        "apparatus_id": self.apparatus_id,
                    }
                }
            },
            None,
        );

        if update_result.await.unwrap().modified_count == 0 {
            None
        } else {
            Some(self.id)
        }
    }
}
