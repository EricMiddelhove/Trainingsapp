use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document, Uuid},
    Client, Collection,
};

pub struct Apparatus {
    id: Uuid,
    name: Option<String>,
    description: Option<String>,

    repetitions: Option<u8>,
    sets: Option<u8>,

    notes: Option<String>,
}

impl Apparatus {
    pub fn new(
        name: &Option<String>,
        description: &Option<String>,
        repetitions: &Option<u8>,
        sets: &Option<u8>,
        notes: &Option<String>,
        id: Option<Uuid>,
    ) -> Apparatus {
        Apparatus {
            id: id.unwrap_or(Uuid::new()),
            name: name.clone(),
            description: description.clone(),
            repetitions: repetitions.clone(),
            sets: sets.clone(),
            notes: notes.clone(),
        }
    }

    pub async fn database_insert(self: Apparatus, client: &Client, userid: String) -> Option<Uuid> {
        let user_collection: Collection<Document> = client.database("prod").collection("users");

        let update_result = user_collection.update_one(
            doc! {
                "_id": ObjectId::parse_str(&userid).expect("Failed to parse user id"),
            },
            doc! {
                "$push":{
                    "apparatus": {
                        "_id": &self.id,
                        "name": self.name,
                        "description": self.description,
                        "repetitions": Bson::Int32(self.repetitions.unwrap() as i32),
                        "sets": Bson::Int32(self.sets.unwrap() as i32),
                        "notes": self.notes,
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

    pub async fn database_update(
        self: Apparatus,
        client: &Client,
        userid: ObjectId,
        apparatusid: Uuid,
    ) -> Option<Uuid> {
        let user_collection: Collection<Document> = client.database("prod").collection("users");

        let bson_reps = match self.repetitions {
            Some(reps) => Bson::Int32(reps as i32),
            None => Bson::Null,
        };
        let bson_sets = match self.sets {
            Some(sets) => Bson::Int32(sets as i32),
            None => Bson::Null,
        };

        let mut update_doc = Document::new();

        if self.name.is_some() {
            update_doc.insert("apparatus.$.name", self.name);
        }
        if self.description.is_some() {
            update_doc.insert("apparatus.$.description", self.description);
        }
        if self.repetitions.is_some() {
            update_doc.insert("apparatus.$.repetitions", bson_reps);
        }
        if self.sets.is_some() {
            update_doc.insert("apparatus.$.sets", bson_sets);
        }
        if self.notes.is_some() {
            update_doc.insert("apparatus.$.notes", self.notes);
        }

        let update_result = user_collection
            .update_one(
                doc! {
                    "_id": &userid,
                    "apparatus._id": apparatusid,
                },
                doc! {
                    "$set": update_doc,
                },
                None,
            )
            .await;

        if update_result.unwrap().modified_count == 0 {
            None
        } else {
            Some(self.id)
        }
    }
}
